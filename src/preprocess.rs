use crate::consts::FRONTMATTER_DELIM;
use regex::Regex;
use std::sync::LazyLock;

static PROPERTY_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<key>[a-zA-Z0-9_-]+)::\s+(?P<value>.*)$").unwrap());

pub fn preprocess_logseq_markdown(md: &str) -> String {
    let mut lines = md.lines().peekable();
    let mut frontmatter: Vec<String> = Vec::new();

    while let Some(line) = lines.peek() {
        if let Some((_, [key, value])) = PROPERTY_REGEX.captures(line).map(|caps| caps.extract()) {
            frontmatter.push(format!("{key}: '{value}'"));
            lines.next();
        } else {
            break;
        }
    }

    let frontmatter_str = if frontmatter.is_empty() {
        String::new()
    } else {
        format!(
            "{}\n{}\n{}\n",
            FRONTMATTER_DELIM,
            frontmatter.join("\n"),
            FRONTMATTER_DELIM
        )
    };
    let remaining_lines: Vec<&str> = lines.collect();

    format!("{}{}", frontmatter_str, remaining_lines.join("\n"))
}
