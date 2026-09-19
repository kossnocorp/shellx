use crate::prelude::*;

#[derive(Args, Debug)]
pub struct ShxCmdRender {
    #[usage()]
    pub code: String,
}

impl Run for ShxCmdRender {
    type Output = Result<()>;

    fn run(self) -> Self::Output {
        println!("{}", self.code);
        Ok(())
    }
}
