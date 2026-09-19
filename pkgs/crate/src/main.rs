mod prelude;

mod parser;
pub use parser::*;

mod node;
pub use node::*;

mod cli;
pub use cli::*;

mod cmd;
pub use cmd::*;

fn main() {
    ShxCli::main();
}
