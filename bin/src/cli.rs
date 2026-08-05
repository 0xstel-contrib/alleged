#[cfg(feature = "caldav")]
use crate::CalDavSyncCommand;
#[cfg(feature = "tui")]
use crate::TuiCommand;
#[cfg(feature = "api")]
use crate::api::ApiCommand;
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
    Api(ApiCommand),
    #[cfg(feature = "caldav")]
    CalDavSync(CalDavSyncCommand),
    #[cfg(feature = "tui")]
    Tui(TuiCommand),
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
