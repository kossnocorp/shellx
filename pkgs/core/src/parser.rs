use crate::prelude::*;

#[derive(Debug)]
pub struct Document<'a> {
    pub nodes: Vec<ShxNode<'a>>,
    pub depth: usize,
}

/// Parse text and paired or self-closing style tags in one pass. The node vector
/// is the arena: no per-node allocations, copied strings, or recursive calls.
/// Names are case-sensitive. Attribute values may be bare or quoted; unsupported
/// attributes and invalid values are ignored. Entity decoding is not supported.
/// Invalid tags and unmatched opening or closing tags remain literal text.
pub fn parse(source_code: &str) -> Document<'_> {
    let bytes = source_code.as_bytes();
    let mut nodes = Vec::new();
    let mut stack = Vec::new();
    let mut depth = 0;
    let mut pos = 0;
    while pos < bytes.len() {
        let start = pos;
        if bytes[pos] != b'<' {
            while pos < bytes.len() && bytes[pos] != b'<' {
                pos += 1;
            }
            nodes.push(ShxNode::Text(&source_code[start..pos]));
            continue;
        }
        pos += 1;
        let closing = bytes.get(pos) == Some(&b'/');
        pos += usize::from(closing);
        let name_start = pos;
        let mut quote = None;
        while pos < bytes.len() {
            let byte = bytes[pos];
            if let Some(delimiter) = quote {
                if byte == delimiter {
                    quote = None;
                }
            } else if matches!(byte, b'\'' | b'"') {
                quote = Some(byte);
            } else if matches!(byte, b'>' | b'<') {
                break;
            }
            pos += 1;
        }
        if bytes.get(pos) != Some(&b'>') {
            nodes.push(ShxNode::Text(&source_code[start..pos]));
            continue;
        }
        let self_closing = !closing && bytes.get(pos - 1) == Some(&b'/');
        let content_end = pos - usize::from(self_closing);
        let mut name_end = name_start;
        while name_end < content_end && !bytes[name_end].is_ascii_whitespace() {
            name_end += 1;
        }
        let name = &source_code[name_start..name_end];
        let tail = &source_code[name_end..content_end];
        pos += 1;
        if closing {
            // Compare spelling as well, so aliases must still be paired exactly.
            if let Some(&(expected, index, tag, attributes)) = stack.last()
                && name == expected
                && tail.trim().is_empty()
            {
                stack.pop();
                nodes[index] = ShxNode::Open(tag, attributes);
                nodes.push(ShxNode::Close);
            } else {
                nodes.push(ShxNode::Text(&source_code[start..pos]));
            }
        } else if let Some(tag) = ShxNodeTag::parse(name) {
            if !self_closing {
                stack.push((name, nodes.len(), tag, parse_attributes(tail)));
                depth = depth.max(stack.len());
                // Promote this to an opening node only when its closing tag appears.
                nodes.push(ShxNode::Text(&source_code[start..pos]));
            }
        } else {
            nodes.push(ShxNode::Text(&source_code[start..pos]));
        }
    }
    Document { nodes, depth }
}

fn parse_attributes(source: &str) -> ShxAttributes {
    let bytes = source.as_bytes();
    let mut attributes = ShxAttributes::default();
    let mut pos = 0;
    while pos < bytes.len() {
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        let start = pos;
        while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() && bytes[pos] != b'=' {
            pos += 1;
        }
        let name = &source[start..pos];
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        let value = if bytes.get(pos) == Some(&b'=') {
            pos += 1;
            while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if matches!(bytes.get(pos), Some(b'\'' | b'"')) {
                let delimiter = bytes[pos];
                pos += 1;
                let start = pos;
                while pos < bytes.len() && bytes[pos] != delimiter {
                    pos += 1;
                }
                let value = &source[start..pos];
                if pos < bytes.len() {
                    pos += 1;
                }
                Some(value)
            } else {
                let start = pos;
                while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
                    pos += 1;
                }
                Some(&source[start..pos])
            }
        } else {
            None
        };
        attributes.set(name, value);
    }
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_unicode_text_and_preserves_nesting() {
        let source = String::from("<red>hé<bold>世界</bold></red>!");
        let doc = parse(&source);
        assert_eq!(doc.depth, 2);
        assert_eq!(
            doc.nodes,
            [
                ShxNode::Open(ShxNodeTag::Red, ShxAttributes::default()),
                ShxNode::Text("hé"),
                ShxNode::Open(ShxNodeTag::Bold, ShxAttributes::default()),
                ShxNode::Text("世界"),
                ShxNode::Close,
                ShxNode::Close,
                ShxNode::Text("!")
            ]
        );
        let ShxNode::Text(text) = doc.nodes[1] else {
            panic!()
        };
        assert_eq!(text.as_ptr(), source[5..].as_ptr());
    }

    #[test]
    fn preserves_invalid_markup_as_text() {
        for source in [
            "<red",
            "<nope>x</nope>",
            "</red>",
            "<red>x</bold>",
            "<red>x",
            "<bright></bold>",
            "<>",
            "<red x>",
            "</>",
            "<<",
            "hé<世界>",
        ] {
            let text: String = parse(source)
                .nodes
                .iter()
                .map(|node| match node {
                    ShxNode::Text(text) => *text,
                    _ => panic!("unexpected style node in {source}"),
                })
                .collect();
            assert_eq!(text, source);
        }
    }

    #[test]
    fn empty_and_self_closing() {
        assert!(parse("").nodes.is_empty());
        assert_eq!(
            parse("a<red/>b").nodes,
            [ShxNode::Text("a"), ShxNode::Text("b")]
        );
    }
}
