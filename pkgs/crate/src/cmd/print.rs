use crate::prelude::*;

#[derive(Args, Debug)]
pub struct ShxCmdRender {
    #[usage()]
    pub code: String,
}

impl Run for ShxCmdRender {
    type Output = Result<()>;

    fn run(self) -> Self::Output {
        let document = parser::parse(&self.code);
        let mut stdout = io::BufWriter::new(io::stdout().lock());
        if std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
            for node in &document.nodes {
                if let ShxNode::Text(text) = node {
                    stdout.write_all(text.as_bytes())?;
                }
            }
        } else {
            render(&document, &mut stdout)?;
        }
        writeln!(stdout)?;
        stdout.flush()?;
        Ok(())
    }
}

// Eight attribute bits and a foreground color (zero means terminal default).
#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct ShxStyle {
    attributes: u8,
    color: u8,
}

impl ShxStyle {
    fn apply(&mut self, tag: ShxNodeTag) {
        match tag {
            ShxNodeTag::Reset => *self = Self::default(),

            ShxNodeTag::Bold
            | ShxNodeTag::Dim
            | ShxNodeTag::Italic
            | ShxNodeTag::Underline
            | ShxNodeTag::Blink
            | ShxNodeTag::Reverse
            | ShxNodeTag::Hidden
            | ShxNodeTag::Strikethrough => {
                self.attributes |= 1 << (tag as u8 - ShxNodeTag::Bold as u8);
            }

            _ => self.color = tag as u8 - ShxNodeTag::Black as u8 + 1,
        }
    }

    fn write_transition(self, previous: Self, out: &mut impl Write) -> io::Result<()> {
        if self == previous {
            return Ok(());
        }

        let reset =
            previous.attributes & !self.attributes != 0 || (previous.color != 0 && self.color == 0);

        let attributes = if reset {
            self.attributes
        } else {
            self.attributes & !previous.attributes
        };

        let color = if reset || self.color != previous.color {
            self.color
        } else {
            0
        };

        // At most reset + eight attributes + foreground, all in one SGR sequence.
        let mut sequence = [0u8; 32];
        sequence[..2].copy_from_slice(b"\x1b[");

        let mut len = 2;
        if reset {
            sequence[len] = b'0';
            len += 1;
        }

        for (bit, code) in b"12345789".iter().copied().enumerate() {
            if attributes & (1 << bit) != 0 {
                if len > 2 {
                    sequence[len] = b';';
                    len += 1;
                }
                sequence[len] = code;
                len += 1;
            }
        }

        if color != 0 {
            if len > 2 {
                sequence[len] = b';';
                len += 1;
            }
            sequence[len] = b'3';
            sequence[len + 1] = b'0' + color - 1;
            len += 2;
        }

        sequence[len] = b'm';
        out.write_all(&sequence[..len + 1])
    }
}

pub(super) fn render(document: &Document<'_>, out: &mut impl Write) -> io::Result<()> {
    let mut stack = Vec::with_capacity(document.depth);
    let mut style = ShxStyle::default();
    let mut emitted = style;

    for node in &document.nodes {
        match *node {
            ShxNode::Text(text) => {
                style.write_transition(emitted, out)?;
                emitted = style;
                out.write_all(text.as_bytes())?;
            }

            ShxNode::Open(tag) => {
                stack.push(style);
                style.apply(tag);
            }

            ShxNode::Close => style = stack.pop().expect("validated nesting"),
        }
    }

    ShxStyle::default().write_transition(emitted, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nested_styles_and_reset() {
        for (source, expected) in [
            ("Hello world", "Hello world"),
            ("<red>x</bold>", "<red>x</bold>"),
            ("<nope><red>x</red></nope>", "<nope>\x1b[31mx\x1b[0m</nope>"),
            ("<red>x</bold>y</red>", "\x1b[31mx</bold>y\x1b[0m"),
            ("<red><bold>x</bold>", "<red>\x1b[1mx\x1b[0m"),
            ("<<red>x</red>", "<\x1b[31mx\x1b[0m"),
            (
                "<red>Hello<bold>Hello</bold></red>",
                "\x1b[31mHello\x1b[1mHello\x1b[0m",
            ),
            (
                "<bright><red>Hello, <italic>world!</italic></red></bright>",
                "\x1b[1;31mHello, \x1b[3mworld!\x1b[0m",
            ),
            (
                "<bright><red>Hello, <reset>world!</reset></red></bright>",
                "\x1b[1;31mHello, \x1b[0mworld!",
            ),
            (
                "<red>a<bold>b</bold>c</red>d",
                "\x1b[31ma\x1b[1mb\x1b[0;31mc\x1b[0md",
            ),
            (
                "<red>a<reset>b</reset>c</red>",
                "\x1b[31ma\x1b[0mb\x1b[31mc\x1b[0m",
            ),
            (
                "<red>a<blue>b</blue>c</red>",
                "\x1b[31ma\x1b[34mb\x1b[31mc\x1b[0m",
            ),
            ("<bold>a<bright>b</bright>c</bold>", "\x1b[1mabc\x1b[0m"),
            ("<red></red><bold/>", ""),
        ] {
            let mut output = Vec::new();
            render(&parser::parse(source), &mut output).unwrap();
            assert_eq!(output, expected.as_bytes(), "{source}");
        }
    }

    #[test]
    fn deep_nesting_is_iterative() {
        let source = format!("{}x{}", "<red>".repeat(10_000), "</red>".repeat(10_000));
        let mut output = Vec::new();
        render(&parser::parse(&source), &mut output).unwrap();
        assert_eq!(output, b"\x1b[31mx\x1b[0m");
    }
}
