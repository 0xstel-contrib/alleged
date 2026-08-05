#[cfg(feature = "caldav")]
use crate::CalDavSync;
#[cfg(feature = "tui")]
use crate::Tui;
#[cfg(feature = "api")]
use crate::api::Api;
use argh::FromArgs;
use std::path::PathBuf;

fn default_graph() -> PathBuf {
    home::home_dir()
        .map(|path| path.join("Documents").join("notes"))
        .expect("ERROR: Failed to detect the home directory!")
}

fn default_exclude() -> Vec<String> {
    vec!["logseq".into(), "contents.md".into()]
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand)]
pub enum CliSubCommand {
    #[cfg(feature = "api")]
    Api(Api),
    #[cfg(feature = "caldav")]
    CalDavSync(CalDavSync),
    #[cfg(feature = "tui")]
    Tui(Tui),
}

/// command-line interface for logseq
#[derive(FromArgs, Debug)]
#[argh(help_triggers("-h", "--help", "help"))]
pub struct Cli {
    #[argh(subcommand)]
    pub command: CliSubCommand,
    /// path to your logseq-og graph [default: $HOME/Documents/notes]
    #[argh(option, short = 'g', default = "default_graph()")]
    pub graph: PathBuf,
    /// paths to exclude [default: logseq,contents.md]
    #[argh(option, short = 'e', default = "default_exclude()")]
    pub exclude: Vec<String>,
}
