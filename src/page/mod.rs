mod error;
mod process;
mod properties;
use std::fmt;

use comrak::format_commonmark;
pub use error::*;
pub use process::*;
pub use properties::*;

use comrak::{Node, nodes::NodeValue};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};

use crate::consts::DEFAULT_COMRAK_OPTIONS;

#[derive(Debug)]
pub struct Page<'a> {
    pub properties: PageProperties,
    pub root: Node<'a>,
}

impl Page<'_> {
    pub fn to_logseq_markdown(&self) -> Result<String, fmt::Error> {
        let mut markdown = String::new();
        format_commonmark(self.root, &DEFAULT_COMRAK_OPTIONS, &mut markdown)?;
        markdown.insert_str(0, &self.properties.to_string());
        Ok(markdown)
    }
}

impl<'a> TryFrom<Node<'a>> for Page<'a> {
    type Error = PageError;

    fn try_from(root: Node<'a>) -> Result<Self, Self::Error> {
        let properties = if let Some(first_child) = root.first_child() {
            if let NodeValue::FrontMatter(ref frontmatter_str) = first_child.data().value {
                let matter: Matter<YAML> = Matter::new();
                let entity: ParsedEntity<RawPageProperties> = matter.parse(frontmatter_str)?;
                entity.data.unwrap_or_default().into()
            } else {
                PageProperties::default()
            }
        } else {
            PageProperties::default()
        };

        // Remove frontmatter node from the AST
        if let Some(first_child) = root.first_child()
            && matches!(first_child.data().value, NodeValue::FrontMatter(_))
        {
            first_child.detach();
        }

        Ok(Self { properties, root })
    }
}
