use crate::prelude::*;

mod print;
use print::*;
mod function;
use function::*;
mod interpolate;
mod source;
use source::*;

#[derive(Subcommands)]
#[usage(run)]
pub enum ShxCmd {
    /// Generates a shell function with zero-based {0} or automatic {} arguments
    Fn(ShxCmdFn),
    /// Renders SHX code
    Render(ShxCmdRender),
    /// Generates Bash/Zsh echo code for eval, with the same arguments as render
    Source(ShxCmdSource),
}
