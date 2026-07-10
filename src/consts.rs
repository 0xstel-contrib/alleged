use comrak::options::{Extension, Options, Render};
use std::sync::LazyLock;

pub const FRONTMATTER_DELIM: &str = "---";

pub static DEFAULT_COMRAK_OPTIONS: LazyLock<Options<'static>> = LazyLock::new(|| {
    let extension = Extension::builder()
        .wikilinks_title_before_pipe(true)
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
