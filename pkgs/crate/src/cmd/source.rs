use crate::prelude::*;

#[derive(Args, Debug)]
pub struct ShxCmdSource {
    #[usage()]
    pub code: String,
    /// Positional values or name=value pairs for placeholders
    #[usage()]
    pub arguments: Vec<String>,
}

impl Run for ShxCmdSource {
    type Output = Result<()>;

    fn run(self) -> Self::Output {
        let document = super::interpolate::interpolate(parser::parse(&self.code), &self.arguments)?;
        let mut rendered = Vec::new();
        super::print::render_output(&document, &mut rendered)?;
        let source = echo(&String::from_utf8(rendered)?);
        let mut stdout = io::stdout().lock();
        stdout.write_all(source.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
}

fn echo(text: &str) -> String {
    let mut escaped = String::new();
    for (index, ch) in text.chars().enumerate() {
        match ch {
            '\'' => escaped.push_str("'\\''"),
            '\\' => escaped.push_str("\\\\"),
            // Encode leading hyphens so echo cannot treat the text as options.
            '-' if index == 0 => escaped.push_str("\\0055"),
            ch if ch.is_ascii_control() => escaped.push_str(&format!("\\0{:03o}", ch as u8)),
            ch => escaped.push(ch),
        }
    }
    format!("command echo -e '{escaped}'\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn generated_echo_preserves_literal_bytes() {
        for text in [
            "",
            "-n",
            "-e",
            "-En",
            "'\" $(exit 9) `exit 9` $HOME; exit 9 #",
            "\\c \\n \\033[31m 100% 世界",
            "line one\nline two\n\n",
            "\0\t\r\x7f",
            "\x1b[32mHello, \x1b[1mworld!\x1b[0m",
        ] {
            let output = Command::new("bash")
                .args(["-c", &echo(text)])
                .output()
                .unwrap();
            assert!(output.status.success(), "{text:?}");
            assert_eq!(output.stdout, format!("{text}\n").as_bytes(), "{text:?}");
        }
    }
}
