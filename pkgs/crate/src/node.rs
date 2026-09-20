/// Nodes live in one contiguous arena in source order, rather than individually
/// allocated trees. Text borrows the input; open/close nodes delimit children.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ShxNode<'source_code> {
    Text(&'source_code str),
    Open(ShxNodeTag),
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
            _ => return None,
        })
    }
}
