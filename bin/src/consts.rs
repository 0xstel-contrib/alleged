use crate::Cli;
use argh::FromArgs;
use std::sync::LazyLock;

pub static HELP: LazyLock<String> = LazyLock::new(|| {
    // Passing `--help` means this will always return `Err(EarlyExit)`, so `unwrap_err()` is OK.
    #[allow(clippy::unwrap_used)]
    let help = Cli::from_args(&[env!("CARGO_PKG_NAME")], &["--help"]).unwrap_err();
    help.output
});

/// Default `radicale` location.
#[cfg(feature = "caldav")]
pub const CALDAV_SERVER: &str = "http://127.0.0.1:5232";
/// Sensible default caldav username.
#[cfg(feature = "caldav")]
pub const CALDAV_USER: &str = "user";
