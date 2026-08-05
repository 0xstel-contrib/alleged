use argh::FromArgs;
use std::path::PathBuf;

/// run an api server to interface with your graph
#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand, name = "api")]
pub struct Api {
    /// path to your logseq graph
    #[argh(option, short = 'g')]
    graph: PathBuf,
    /// port to run the api server on [default: 9001]
    #[argh(option, short = 'p', default = "9001")]
    port: usize,
}
