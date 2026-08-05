use argh::FromArgs;

/// run an api server to interface with your graph
#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand, name = "api")]
pub struct Api {
    /// port to run the api server on [default: 9001]
    #[argh(option, short = 'p', default = "9001")]
    port: usize,
}
