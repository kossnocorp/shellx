use crate::prelude::*;

pub(super) use shellx_core::render::{render, render_output};

#[derive(Args, Debug)]
pub struct ShxCmdPrint {
    #[usage()]
    pub code: String,
    /// Positional values or name=value pairs for placeholders
    #[usage()]
    pub arguments: Vec<String>,
}

impl Run for ShxCmdPrint {
    type Output = Result<()>;

    fn run(self) -> Self::Output {
        let document = super::interpolate::interpolate(parser::parse(&self.code), &self.arguments)?;
        let mut stdout = io::BufWriter::new(io::stdout().lock());
        render_output(&document, &mut stdout)?;
        writeln!(stdout)?;
        stdout.flush()?;
        Ok(())
    }
}
