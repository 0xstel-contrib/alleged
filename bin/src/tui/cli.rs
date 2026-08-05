use argh::FromArgs;

/// terminal user interface for logseq [default]
#[derive(FromArgs, PartialEq, Eq, Debug, Default)]
#[argh(subcommand, name = "tui")]
pub struct TuiCommand {}
