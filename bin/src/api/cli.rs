use argh::FromArgs;

fn default_host() -> String {
    "127.0.0.1".into()
}

/// run an api server to interface with your graph
#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand, name = "api")]
pub struct Api {
    /// host to run the api server on [default: 127.0.0.1]
    #[argh(option, short = 'h', default = "default_host()")]
    pub host: String,
    /// port to run the api server on [default: 9001]
    #[argh(option, short = 'p', default = "9001")]
    pub port: u16,
}
