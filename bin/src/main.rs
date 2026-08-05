#[cfg(feature = "api")]
mod api;
#[cfg(feature = "caldav")]
mod caldav;
mod cli;
#[cfg(feature = "api")]
pub use api::*;
#[cfg(feature = "caldav")]
pub use caldav::*;
pub use cli::*;

use anyhow::{Result, anyhow};
use argh::FromArgs;
use std::env;

pub struct Tui;

fn main() -> Result<()> {
    let args_env: Vec<String> = env::args().collect();
    let mut args_argh: Vec<&str> = args_env.iter().map(String::as_str).collect();
    let current_exe = args_argh.remove(0);

    match Cli::from_args(&[current_exe], &args_argh) {
        Ok(args) => {
            match args.command {
                #[cfg(feature = "api")]
                CliSubcommand::Api(api) => todo!("{api:#?}"),
            }

            Ok(())
        }
        Err(early_exit) => {
            let help = Cli::from_args(&[current_exe], &["--help"]).unwrap_err();
            println!("{}", help.output);

            Err(anyhow!(early_exit.output.trim().to_string()))
        }
    }
}
