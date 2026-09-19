use crate::prelude::*;

mod print;
use print::*;
mod function;
use function::*;

#[derive(Subcommands)]
#[usage(run)]
pub enum ShxCmd {
    /// Generates a shell function with zero-based {0} or automatic {} arguments
    Fn(ShxCmdFn),
    /// Renders SHX code
    Render(ShxCmdRender),
}
