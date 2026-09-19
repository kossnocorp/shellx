use crate::prelude::*;

mod print;
use print::*;

#[derive(Subcommands)]
#[usage(run)]
pub enum ShxCmd {
    /// Renders SHX code
    Render(ShxCmdRender),
}
