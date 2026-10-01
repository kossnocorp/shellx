mod prelude;

pub use shellx_core::{Document, ShxAttributes, ShxNode, ShxNodeTag, parser};

mod cli;
pub use cli::*;

mod cmd;
pub use cmd::*;

fn main() {
    ShxCli::main();
}
