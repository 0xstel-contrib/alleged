use comrak::options::{Extension, Options, Render};
use regex::Regex;
use std::sync::LazyLock;

pub const GRAPH_LAYOUT: [&str; 3] = ["journals", "logseq", "pages"];
pub const FRONTMATTER_DELIM: &str = "---";
pub const SCHEDULED_DELIM: &str = "SCHEDULED:";
pub const LOGSEQ_TOKENS: [&str; 1] = [SCHEDULED_DELIM];

pub static SCHEDULED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"SCHEDULED:\s*<(\d{4}-\d{2}-\d{2})\s+([A-Za-z]{3})(?:\s+(\d{1,2}:\d{2}))?(?:\s+([\.\+]*\+\d+[ymwdh]))?>$"
    ).unwrap()
});
pub static DEFAULT_COMRAK_OPTIONS: LazyLock<Options<'static>> = LazyLock::new(|| {
    let extension = Extension::builder()
        .strikethrough(true)
        .tasklist(true)
        .footnotes(true)
        .autolink(true)
        .underline(true)
        .front_matter_delimiter(FRONTMATTER_DELIM.to_string())
        .build();
    let render = Render::builder()
        .experimental_minimize_commonmark(true)
        .build();

    Options {
        extension,
        render,
        ..Default::default()
    }
});
