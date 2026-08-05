use argh::FromArgs;

/// terminal user interface for logseq
#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand, name = "tui")]
pub struct TuiCommand {}
