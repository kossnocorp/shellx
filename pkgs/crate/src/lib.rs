//! Compile-time styled text for Rust.
//!
//! Add `shellx = { version = "0.5", default-features = false }` to use just
//! the library without the CLI dependencies.
//!
//! ```
//! let text = shellx::render!({
//!     <green bold>Hello, <bold>{name}!</bold></green>
//! }, name = "Sasha");
//! shellx::print!({<green>{text}</green>});
//! ```
//!
//! Markup and ANSI transitions are compiled into format strings. At runtime,
//! only Rust formatting and (for [`print!`]) stdout output remain. Values are
//! never parsed as markup. A nonempty `NO_COLOR` selects the precompiled plain
//! text version at runtime.
//!
//! Placeholders follow Rust formatting syntax: `{name}`, `{0}`, `{}`, and
//! `{value:>8}`. Escape literal braces with `{{` and `}}`. Arguments and implicit
//! captures work just like `format!`. The CLI separately uses double braces.
//!
//! Braced markup preserves spaces between tokens and folds line breaks to a
//! space, trimming outer whitespace. Whitespace inside format fields is ignored.
//! Rust tokenization rules apply; use a string
//! literal for arbitrary text, exact whitespace, or escape sequences:
//!
//! ```
//! let text = shellx::render!("<blue>{:04}</blue>\n", 42);
//! ```
//!
//! `render!` returns a `String`. `print!` returns `()` and writes directly to
//! stdout without an intermediate string or trailing newline, like `std::print!`.
//!
//! Invalid Rust formatting is rejected at compile time:
//! ```compile_fail
//! shellx::render!({<red>{missing}</red>});
//! ```
//! ```compile_fail
//! shellx::render!("<red>{</red>");
//! ```

pub use shellx_macros::{print, render};
