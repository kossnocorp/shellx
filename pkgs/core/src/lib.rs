//! Shared markup parsing and ANSI rendering for the CLI and macro compiler.

mod node;
pub use node::*;
pub mod parser;
pub use parser::Document;
pub mod render;

mod prelude {
    pub(crate) use crate::*;
    pub(crate) use std::io::{self, Write};
}
