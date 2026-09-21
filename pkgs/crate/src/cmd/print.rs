use crate::prelude::*;

#[derive(Args, Debug)]
pub struct ShxCmdPrint {
    #[usage()]
    pub code: String,
    /// Positional values or name=value pairs for placeholders
    #[usage()]
    pub arguments: Vec<String>,
}

impl Run for ShxCmdPrint {
    type Output = Result<()>;

    fn run(self) -> Self::Output {
        let document = super::interpolate::interpolate(parser::parse(&self.code), &self.arguments)?;
        let mut stdout = io::BufWriter::new(io::stdout().lock());
        render_output(&document, &mut stdout)?;
        writeln!(stdout)?;
        stdout.flush()?;
        Ok(())
    }
}

pub(super) fn render_output(document: &Document<'_>, out: &mut impl Write) -> io::Result<()> {
    if std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
        for node in &document.nodes {
            if let ShxNode::Text(text) = node {
                out.write_all(text.as_bytes())?;
            }
        }
        Ok(())
    } else {
        render(document, out)
    }
}

// Eight attribute bits and palette colors (zero means terminal default).
#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct ShxStyle {
    attributes: u8,
    color: u8,
    background: u8,
    bright: bool,
}

impl ShxStyle {
    fn apply(&mut self, tag: ShxNodeTag) {
        match tag {
            ShxNodeTag::Reset => *self = Self::default(),
            ShxNodeTag::Bright => self.bright = true,
            ShxNodeTag::Span => {}

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

    fn apply_attributes(&mut self, attributes: ShxAttributes) {
        self.attributes = (self.attributes & !(attributes.mask as u8)) | attributes.flags as u8;
        if attributes.mask & ShxAttributes::BRIGHT != 0 {
            self.bright = attributes.flags & ShxAttributes::BRIGHT != 0;
        }
        if attributes.fg != 0 {
            self.color = attributes.fg;
        }
        if attributes.bg != 0 {
            self.background = attributes.bg;
        }
    }

    fn palette(self, color: u8) -> u8 {
        if self.bright && (1..=8).contains(&color) {
            color + 8
        } else {
            color
        }
    }

    fn write_transition(self, previous: Self, out: &mut impl Write) -> io::Result<()> {
        let foreground = self.palette(self.color);
        let previous_foreground = previous.palette(previous.color);
        let background = self.palette(self.background);
        let previous_background = previous.palette(previous.background);
        if self.attributes == previous.attributes
            && foreground == previous_foreground
            && background == previous_background
        {
            return Ok(());
        }

        let reset = previous.attributes & !self.attributes != 0
            || (previous_foreground != 0 && foreground == 0)
            || (previous_background != 0 && background == 0);

        let attributes = if reset {
            self.attributes
        } else {
            self.attributes & !previous.attributes
        };

        let color = if reset || foreground != previous_foreground {
            foreground
        } else {
            0
        };

        let background = if reset || background != previous_background {
            background
        } else {
            0
        };

        // At most reset + eight attributes + both colors, in one SGR sequence.
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
            // Colors 1–8 use SGR 30–37; bright colors 9–16 use SGR 90–97.
            sequence[len] = if color > 8 { b'9' } else { b'3' };
            sequence[len + 1] = b'0' + (color - 1) % 8;
            len += 2;
        }

        if background != 0 {
            if len > 2 {
                sequence[len] = b';';
                len += 1;
            }
            // Base backgrounds use 40–47, bright backgrounds use 100–107.
            if background > 8 {
                sequence[len..len + 2].copy_from_slice(b"10");
                len += 2;
            } else {
                sequence[len] = b'4';
                len += 1;
            }
            sequence[len] = b'0' + (background - 1) % 8;
            len += 1;
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

            ShxNode::Open(tag, attributes) => {
                stack.push(style);
                style.apply(tag);
                style.apply_attributes(attributes);
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
    fn renders_attributes_and_restores_inherited_styles() {
        for (source, expected) in [
            ("<span>x</span>", "x"),
            (
                "<span fg=red bg=green bold dim italic>x</span>",
                "\x1b[1;2;3;31;42mx\x1b[0m",
            ),
            ("<red bold dim bg=green>x</red>", "\x1b[1;2;31;42mx\x1b[0m"),
            ("<span fg=gray bg=gray>x</span>", "\x1b[90;100mx\x1b[0m"),
            (
                "<span fg=white bg=blue bright>x</span>",
                "\x1b[97;104mx\x1b[0m",
            ),
            (
                "<span bold=true dim=false italic='true' fg=\"red\">x</span>",
                "\x1b[1;3;31mx\x1b[0m",
            ),
            (
                "<span underline blink reverse hidden strikethrough>x</span>",
                "\x1b[4;5;7;8;9mx\x1b[0m",
            ),
            (
                "<red bold>a<span bold=false bg=blue>b</span>c</red>",
                "\x1b[1;31ma\x1b[0;31;44mb\x1b[0;1;31mc\x1b[0m",
            ),
            (
                "<span fg=red bg=green bright>a<span bright=false>b</span>c</span>",
                "\x1b[91;102ma\x1b[31;42mb\x1b[91;102mc\x1b[0m",
            ),
            (
                "<span bg=red>a<reset>b</reset>c</span>",
                "\x1b[41ma\x1b[0mb\x1b[41mc\x1b[0m",
            ),
            (
                "<red unknown='x > < 世界' italic=maybe bg=nope bold=1>x</red>",
                "\x1b[31mx\x1b[0m",
            ),
            ("<span fg = 'cyan' bold = true bold=false italic/>x", "x"),
            (
                "<span fg=red fg=green bold bold=false>x</span>",
                "\x1b[32mx\x1b[0m",
            ),
            ("<bold bold=false>x</bold>", "x"),
            (
                "<span bold dim italic underline blink reverse hidden strikethrough bright fg=white bg=white>x</span>",
                "\x1b[1;2;3;4;5;7;8;9;97;107mx\x1b[0m",
            ),
        ] {
            let mut output = Vec::new();
            render(&parser::parse(source), &mut output).unwrap();
            assert_eq!(output, expected.as_bytes(), "{source}");
        }
    }

    #[test]
    fn boolean_false_disables_each_inherited_attribute() {
        for (name, code) in [
            ("bold", 1),
            ("dim", 2),
            ("italic", 3),
            ("underline", 4),
            ("blink", 5),
            ("reverse", 7),
            ("hidden", 8),
            ("strikethrough", 9),
        ] {
            let source = format!("<span {name}>a<span {name}=false>b</span>c</span>");
            let mut output = Vec::new();
            render(&parser::parse(&source), &mut output).unwrap();
            assert_eq!(
                output,
                format!("\x1b[{code}ma\x1b[0mb\x1b[{code}mc\x1b[0m").as_bytes()
            );
        }
    }

    #[test]
    fn renders_nested_styles_and_reset() {
        for (source, expected) in [
            ("Hello world", "Hello world"),
            ("<gray>gray</gray>", "\x1b[90mgray\x1b[0m"),
            (
                "<bright><black>gray</black></bright>",
                "\x1b[90mgray\x1b[0m",
            ),
            (
                "<bright-black>x</bright-black>",
                "<bright-black>x</bright-black>",
            ),
            ("<bright>plain</bright>", "plain"),
            (
                "<black>a<bright>b</bright>c</black>",
                "\x1b[30ma\x1b[90mb\x1b[30mc\x1b[0m",
            ),
            (
                "<bright><red>a<reset>b</reset>c</red></bright>",
                "\x1b[91ma\x1b[0mb\x1b[91mc\x1b[0m",
            ),
            (
                "<bright><bold><gray>x</gray></bold></bright>",
                "\x1b[1;90mx\x1b[0m",
            ),
            (
                "<red>a<gray>b</gray>c</red>",
                "\x1b[31ma\x1b[90mb\x1b[31mc\x1b[0m",
            ),
            (
                "<gray>a<reset>b</reset><bold>c</bold>d</gray>",
                "\x1b[90ma\x1b[0mb\x1b[1;90mc\x1b[0;90md\x1b[0m",
            ),
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
                "\x1b[91mHello, \x1b[3mworld!\x1b[0m",
            ),
            (
                "<bright><red>Hello, <reset>world!</reset></red></bright>",
                "\x1b[91mHello, \x1b[0mworld!",
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
    fn bright_palette_is_independent_of_tag_order() {
        for (index, color) in [
            "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
        ]
        .into_iter()
        .enumerate()
        {
            for source in [
                format!("<bright><{color}>x</{color}></bright>"),
                format!("<{color}><bright>x</bright></{color}>"),
            ] {
                let mut output = Vec::new();
                render(&parser::parse(&source), &mut output).unwrap();
                assert_eq!(
                    output,
                    format!("\x1b[{}mx\x1b[0m", 90 + index).as_bytes(),
                    "{source}"
                );
            }
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
