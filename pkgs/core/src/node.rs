/// Nodes live in one contiguous arena in source order, rather than individually
/// allocated trees. Text borrows the input; open/close nodes delimit children.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ShxNode<'source_code> {
    Text(&'source_code str),
    Open(ShxNodeTag, ShxAttributes),
    Close,
}

/// Compact tags supported by the SHX grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ShxNodeTag {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    Bright,
    Span,
}

impl ShxNodeTag {
    pub(super) fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "reset" => Self::Reset,
            "bold" => Self::Bold,
            "bright" => Self::Bright,
            "dim" => Self::Dim,
            "italic" => Self::Italic,
            "underline" => Self::Underline,
            "blink" => Self::Blink,
            "reverse" => Self::Reverse,
            "hidden" => Self::Hidden,
            "strikethrough" => Self::Strikethrough,
            "black" => Self::Black,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" => Self::Magenta,
            "cyan" => Self::Cyan,
            "white" => Self::White,
            "gray" => Self::Gray,
            "span" => Self::Span,
            _ => return None,
        })
    }
}

/// Inline overrides: a mask distinguishes absent booleans from explicit false.
/// Colors are 1–8 for the base palette, 9 for gray, and 0 for inheritance.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ShxAttributes {
    pub mask: u16,
    pub flags: u16,
    pub fg: u8,
    pub bg: u8,
}

impl ShxAttributes {
    pub const BRIGHT: u16 = 1 << 8;

    pub(super) fn set(&mut self, name: &str, value: Option<&str>) {
        if matches!(name, "fg" | "bg") {
            let Some(tag) = value.and_then(ShxNodeTag::parse) else {
                return;
            };
            if (ShxNodeTag::Black as u8..=ShxNodeTag::Gray as u8).contains(&(tag as u8)) {
                let color = tag as u8 - ShxNodeTag::Black as u8 + 1;
                if name == "fg" {
                    self.fg = color;
                } else {
                    self.bg = color;
                }
            }
            return;
        }
        let bit = match ShxNodeTag::parse(name) {
            Some(ShxNodeTag::Bright) => Self::BRIGHT,
            Some(tag)
                if (ShxNodeTag::Bold as u8..=ShxNodeTag::Strikethrough as u8)
                    .contains(&(tag as u8)) =>
            {
                1 << (tag as u8 - ShxNodeTag::Bold as u8)
            }
            _ => return,
        };
        match value {
            None | Some("true") => self.flags |= bit,
            Some("false") => self.flags &= !bit,
            _ => return,
        }
        self.mask |= bit;
    }
}
