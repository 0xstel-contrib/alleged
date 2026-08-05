#[cfg(feature = "api")]
use crate::api::Api;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand)]
pub enum CliSubcommand {
    #[cfg(feature = "api")]
    Api(Api),
}

/// command-line interface for logseq
#[derive(FromArgs, Debug)]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct Cli {
    #[argh(subcommand)]
    pub command: CliSubcommand,
}
