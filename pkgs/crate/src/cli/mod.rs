use crate::prelude::*;

#[derive(Cli)]
#[usage(
    run,
    bin = "shx",
    about = "SHX, the JSX for shell scripts",
    default_subcommand = "render"
)]
pub struct ShxCli {
    #[usage(subcommand)]
    pub command: ShxCmd,
}

impl ShxCli {
    pub fn main() {
        ShxCli::parse().run().unwrap_or_else(|err| {
            eprintln!("Error: {:?}", err);
            std::process::exit(1);
        });
    }
}
