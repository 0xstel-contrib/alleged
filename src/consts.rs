use comrak::options::{Extension, Options, Render};
#[cfg(feature = "regex")]
use regex::Regex;
#[cfg(feature = "regex-lite")]
use regex_lite::Regex;
use std::sync::LazyLock;

pub const GRAPH_LAYOUT: [&str; 3] = ["journals", "logseq", "pages"];
pub const FRONTMATTER_DELIM: &str = "---";
pub const SCHEDULED_DELIM: &str = "SCHEDULED:";
pub const LOGSEQ_TOKENS: [&str; 1] = [SCHEDULED_DELIM];

pub static PROPERTY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<key>[a-zA-Z0-9_-]+)::\s+(?P<value>.*)$").unwrap());
pub static SCHEDULED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"SCHEDULED:\s*<(\d{4}-\d{2}-\d{2})\s+([A-Za-z]{3})(?:\s+(\d{1,2}:\d{2}))?(?:\s+([\.\+]*\+\d+[ymwdh]))?>$"
    ).unwrap()
});
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
