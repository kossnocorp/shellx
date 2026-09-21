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
    Print(ShxCmdPrint),
    /// Generates Bash/Zsh echo code for eval, with the same arguments as print
    Source(ShxCmdSource),
}
