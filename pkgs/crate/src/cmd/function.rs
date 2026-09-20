use crate::prelude::*;

#[derive(Args, Debug)]
pub struct ShxCmdFn {
    #[usage()]
    pub name: String,
    #[usage()]
    pub code: String,
}

impl Run for ShxCmdFn {
    type Output = Result<()>;

    fn run(self) -> Self::Output {
        let source = compile(&self.name, &self.code)?;
        let mut stdout = io::stdout().lock();
        stdout.write_all(source.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

fn quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

// Escape only template literals for printf; argument values are always passed via %s.
fn template(text: &str, next_index: &mut usize, arguments: &mut Vec<usize>) -> Result<String> {
    let mut format = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                format.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                format.push('}');
            }
            '{' => {
                let mut placeholder = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some(ch) => placeholder.push(ch),
                        None => anyhow::bail!("Unclosed placeholder; use {{{{ for a literal brace"),
                    }
                }
                let index = if placeholder.is_empty() {
                    *next_index
                } else {
                    anyhow::ensure!(
                        placeholder.bytes().all(|byte| byte.is_ascii_digit()),
                        "Invalid placeholder: {{{placeholder}}}; expected an index or {{}}"
                    );
                    placeholder.parse::<usize>().map_err(|_| {
                        anyhow::anyhow!("Placeholder index is too large: {placeholder}")
                    })?
                };
                let position = index
                    .checked_add(1)
                    .ok_or_else(|| anyhow::anyhow!("Placeholder index is too large: {index}"))?;
                if placeholder.is_empty() {
                    *next_index = position;
                }
                arguments.push(position);
                format.push_str("%s");
            }
            '}' => anyhow::bail!("Unmatched closing brace; use }}}} for a literal brace"),
            '%' => format.push_str("%%"),
            '\\' => format.push_str("\\\\"),
            ch => format.push(ch),
        }
    }
    Ok(format)
}

fn compile(name: &str, code: &str) -> Result<String> {
    anyhow::ensure!(identifier(name), "Invalid function name: {name}");
    anyhow::ensure!(
        !matches!(
            name,
            "if" | "then"
                | "else"
                | "elif"
                | "fi"
                | "do"
                | "done"
                | "case"
                | "esac"
                | "while"
                | "until"
                | "for"
                | "in"
                | "function"
                | "select"
                | "time"
                | "coproc"
                | "command"
                | "return"
        ),
        "Reserved shell function name: {name}"
    );
    anyhow::ensure!(!code.contains('\0'), "Templates cannot contain NUL bytes");
    let document = parser::parse(code);
    let mut next_index = 0;
    let mut arguments = Vec::new();
    let texts = document
        .nodes
        .iter()
        .map(|node| match node {
            ShxNode::Text(text) => template(text, &mut next_index, &mut arguments),
            _ => Ok(String::new()),
        })
        .collect::<Result<Vec<_>>>()?;
    let formatted = Document {
        depth: document.depth,
        nodes: document
            .nodes
            .iter()
            .zip(&texts)
            .map(|(node, text)| match node {
                ShxNode::Text(_) => ShxNode::Text(text),
                node => *node,
            })
            .collect(),
    };
    let plain = texts.concat();
    let mut colored = Vec::new();
    super::print::render(&formatted, &mut colored)?;
    let colored = String::from_utf8(colored)?;
    let args = arguments
        .iter()
        .map(|index| format!(" \"${{{index}}}\""))
        .collect::<String>();
    // Explicit indexes can repeat, appear out of order, or skip positions.
    let count = arguments.iter().copied().max().unwrap_or(0);
    let suffix = if count == 1 { "" } else { "s" };
    let usage = format!("Usage: {name} ({count} argument{suffix})");
    Ok(format!(
        "{name}() {{\n  if [ \"$#\" -ne {count} ]; then\n    command printf '%s\\n' {usage} >&2\n    return 2\n  fi\n  if [ -n \"${{NO_COLOR:-}}\" ]; then\n    command printf {plain}{args}\n  else\n    command printf {colored}{args}\n  fi\n}}\n",
        usage = quote(&usage),
        plain = quote(&(plain + "\\n")),
        colored = quote(&(colored + "\\n")),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Output};

    fn call(code: &str, args: &[&str], no_color: bool) -> Output {
        let source = compile("greet", code).unwrap();
        Command::new("bash")
            .args([
                "-c",
                &format!("set -euo pipefail\n{source}\ngreet \"$@\""),
                "test",
            ])
            .args(args)
            .env("NO_COLOR", if no_color { "1" } else { "" })
            .output()
            .unwrap()
    }

    #[test]
    fn attributes_work_in_colored_and_plain_functions() {
        let code = "<span fg=red bg=gray bold>{0}</span>";
        assert_eq!(
            call(code, &["Hello"], false).stdout,
            b"\x1b[1;31;100mHello\x1b[0m\n"
        );
        assert_eq!(call(code, &["Hello"], true).stdout, b"Hello\n");
    }

    #[test]
    fn inferred_arguments_and_nested_styles() {
        let code = "<red>Hello, <green>{0}</green>! {1}, {0}</red>";
        let output = call(code, &["Sasha", "world"], false);
        assert!(output.status.success());
        assert_eq!(
            output.stdout,
            b"\x1b[31mHello, \x1b[32mSasha\x1b[31m! world, Sasha\x1b[0m\n"
        );
        assert_eq!(
            call(code, &["Sasha", "world"], true).stdout,
            b"Hello, Sasha! world, Sasha\n"
        );
    }

    #[test]
    fn literals_and_values_are_not_interpreted() {
        let value = "<red>'\" $(exit 9) `exit 9` %s \\n\n世界";
        let output = call(
            "'\" $(exit 9) `exit 9` 100% \\n {{name}} {0}",
            &[value],
            true,
        );
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("'\" $(exit 9) `exit 9` 100% \\n {{name}} {value}\n")
        );
    }

    #[test]
    fn checks_argument_count_including_empty_values() {
        for args in [vec![], vec!["a", "b"]] {
            let output = call("{0}", &args, true);
            assert_eq!(output.status.code(), Some(2));
            assert!(output.stdout.is_empty());
            assert_eq!(output.stderr, b"Usage: greet (1 argument)\n");
        }
        assert_eq!(call("{0}", &[""], true).stdout, b"\n");
        assert_eq!(call("constant", &[], true).stdout, b"constant\n");
    }

    #[test]
    fn rejects_invalid_definitions() {
        for name in [
            "",
            "1hello",
            "say-hello",
            "x; exit",
            "if",
            "for",
            "command",
            "return",
        ] {
            assert!(compile(name, "hello").is_err());
        }
        for code in [
            "{",
            "}",
            "{name}",
            "{-1}",
            "{+1}",
            "{ 0}",
            "{not valid}",
            "{name",
            "\0",
            "{999999999999999999999999999999}",
        ] {
            assert!(compile("greet", code).is_err());
        }
        assert!(compile("greet", &format!("{{{}}}", usize::MAX)).is_err());
    }

    #[test]
    fn automatic_indexes_span_style_nodes() {
        assert_eq!(
            compile("greet", "{}").unwrap(),
            compile("greet", "{0}").unwrap()
        );
        let output = call("<red>{}</red> <green>{}</green> {}", &["a", "b", "c"], true);
        assert!(output.status.success());
        assert_eq!(output.stdout, b"a b c\n");
        let output = call("{2} {} {0} {} {{}} {{0}}", &["a", "b", "c"], true);
        assert!(output.status.success());
        assert_eq!(output.stdout, b"c a a b {} {0}\n");
    }

    #[test]
    fn explicit_indexes_determine_arity_and_support_ten_or_more_arguments() {
        let output = call("{2} {0} {2}", &["a", "b", "c"], true);
        assert!(output.status.success());
        assert_eq!(output.stdout, b"c a c\n");
        assert_eq!(call("{2}", &["a"], true).status.code(), Some(2));
        let output = call(
            "{10} {9} {0}",
            &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            true,
        );
        assert!(output.status.success());
        assert_eq!(output.stdout, b"10 9 0\n");
    }
}
