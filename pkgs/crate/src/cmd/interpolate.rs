use crate::prelude::*;
use std::collections::HashMap;

/// Expand only text nodes, borrowing both literal slices and argument values.
/// Substitutions are never reparsed as markup or as further placeholders.
pub(super) fn interpolate<'a>(
    document: Document<'a>,
    arguments: &'a [String],
) -> Result<Document<'a>> {
    if !document
        .nodes
        .iter()
        .any(|node| matches!(node, ShxNode::Text(text) if text.contains(['{', '}'])))
    {
        return Ok(document);
    }

    let mut positional = Vec::new();
    let mut named = HashMap::new();
    for argument in arguments {
        if let Some((name, value)) = argument.split_once('=')
            && super::function::identifier(name)
        {
            named.insert(name, value);
        } else {
            positional.push(argument.as_str());
        }
    }

    let mut nodes = Vec::with_capacity(document.nodes.len());
    let mut next_index = 0;
    for node in document.nodes {
        let ShxNode::Text(text) = node else {
            nodes.push(node);
            continue;
        };
        let bytes = text.as_bytes();
        let mut pos = 0;
        while pos < bytes.len() {
            let start = pos;
            while pos < bytes.len() && !matches!(bytes[pos], b'{' | b'}') {
                pos += 1;
            }
            if start != pos {
                nodes.push(ShxNode::Text(&text[start..pos]));
            }
            if pos == bytes.len() {
                break;
            }
            let brace = bytes[pos];
            if bytes.get(pos + 1) == Some(&brace) {
                nodes.push(ShxNode::Text(&text[pos..pos + 1]));
                pos += 2;
                continue;
            }
            anyhow::ensure!(
                brace == b'{',
                "Unmatched closing brace; use }}}} for a literal brace"
            );
            pos += 1;
            let start = pos;
            while pos < bytes.len() && bytes[pos] != b'}' {
                pos += 1;
            }
            anyhow::ensure!(
                pos < bytes.len(),
                "Unclosed placeholder; use {{{{ for a literal brace"
            );
            let placeholder = &text[start..pos];
            let value = if placeholder.is_empty()
                || placeholder.bytes().all(|byte| byte.is_ascii_digit())
            {
                let index = if placeholder.is_empty() {
                    let index = next_index;
                    next_index += 1;
                    index
                } else {
                    placeholder.parse::<usize>().map_err(|_| {
                        anyhow::anyhow!("Placeholder index is too large: {placeholder}")
                    })?
                };
                *positional.get(index).ok_or_else(|| {
                    anyhow::anyhow!("Missing positional argument at index {index}")
                })?
            } else {
                anyhow::ensure!(
                    super::function::identifier(placeholder),
                    "Invalid placeholder: {{{placeholder}}}"
                );
                *named
                    .get(placeholder)
                    .ok_or_else(|| anyhow::anyhow!("Missing named argument: {placeholder}"))?
            };
            // Empty values must not cause a style transition with no visible text.
            if !value.is_empty() {
                nodes.push(ShxNode::Text(value));
            }
            pos += 1;
        }
    }
    Ok(Document {
        nodes,
        depth: document.depth,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn output(source: &str, args: &[&str]) -> Result<String> {
        let args: Vec<_> = args.iter().map(|arg| (*arg).to_owned()).collect();
        let document = interpolate(parser::parse(source), &args)?;
        let mut bytes = Vec::new();
        super::super::print::render(&document, &mut bytes)?;
        Ok(String::from_utf8(bytes)?)
    }

    #[test]
    fn supports_all_placeholder_forms() {
        for (source, args, expected) in [
            (
                "Hello, {}! {}",
                vec!["Sasha", "How are you?"],
                "Hello, Sasha! How are you?",
            ),
            (
                "Hello, {1}! {0}",
                vec!["How are you?", "Sasha"],
                "Hello, Sasha! How are you?",
            ),
            (
                "{hello}, {name}! {question}",
                vec!["name=Sasha", "hello=Hi", "question=How are you?"],
                "Hi, Sasha! How are you?",
            ),
            (
                "{1} {} {name} {} {0}",
                vec!["first", "name=named", "second"],
                "second first named second first",
            ),
            ("{{{0}}} {{}}", vec!["世界"], "{世界} {}"),
            ("{name}/{name}", vec!["name=a=b"], "a=b/a=b"),
            ("a{}b{name}c", vec!["", "name="], "abc"),
            ("plain", vec![], "plain"),
        ] {
            assert_eq!(output(source, &args).unwrap(), expected, "{source}");
        }
    }

    #[test]
    fn substitutions_inherit_styles_without_becoming_markup() {
        assert_eq!(
            output(
                "<red>{}</red> <span bg=blue>{}</span>",
                &["<bold>{name}</bold>", "世界"]
            )
            .unwrap(),
            "\x1b[31m<bold>{name}</bold>\x1b[0m \x1b[44m世界\x1b[0m"
        );
        assert_eq!(output("<red>{}</red>", &[""]).unwrap(), "");
    }

    #[test]
    fn invalid_or_missing_placeholders_are_errors() {
        for source in [
            "{}",
            "{0}",
            "{name}",
            "{",
            "}",
            "{not a name}",
            "{999999999999999999999999999999999}",
        ] {
            assert!(output(source, &[]).is_err(), "{source}");
        }
    }
}
