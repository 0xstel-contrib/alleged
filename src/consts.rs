use comrak::options::{Extension, Options, Render};
#[cfg(feature = "regex")]
use regex::Regex;
#[cfg(feature = "regex-lite")]
use regex_lite::Regex;
use std::sync::LazyLock;
use time::{format_description::StaticFormatDescription, macros::format_description};

pub static COMRAK_OPTIONS: LazyLock<Options<'static>> = LazyLock::new(|| Options {
    extension: Extension {
        strikethrough: true,
        tasklist: true,
        footnotes: true,
        autolink: true,
        underline: true,
        ..Default::default()
    },
    render: Render {
        experimental_minimize_commonmark: true,
        ..Default::default()
    },
    ..Default::default()
});

pub static DEFAULT_EXCLUDE: [&str; 2] = ["logseq", "contents.md"];

pub static JOURNAL_FORMAT: StaticFormatDescription = format_description!("[year]_[month]_[day]");
pub static DATE_FORMAT: StaticFormatDescription = format_description!("[year]-[month]-[day]");
pub static TIME_FORMAT: StaticFormatDescription = format_description!("[hour]:[minute]");

#[allow(clippy::unwrap_used)]
pub static PROPERTY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<key>[a-zA-Z0-9_-]+)::\s+(?P<value>.*)$").unwrap());
#[allow(clippy::unwrap_used)]
pub static SCHEDULED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"SCHEDULED:\s*<(\d{4}-\d{2}-\d{2})\s+([A-Za-z]{3})(?:\s+(\d{1,2}:\d{2}))?(?:\s+([\.\+]*\+\d+[ymwdh]))?>$"
    ).unwrap()
});

pub const GRAPH_LAYOUT: [&str; 3] = ["journals", "logseq", "pages"];
pub const SCHEDULED_DELIM: &str = "SCHEDULED:";
pub const LOGSEQ_TOKENS: [&str; 1] = [SCHEDULED_DELIM];
