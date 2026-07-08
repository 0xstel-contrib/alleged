mod error;
mod preprocess;
mod properties;
pub use error::*;
pub use preprocess::*;
pub use properties::*;

use comrak::{Node, nodes::NodeValue};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};

#[derive(Debug)]
pub struct Page<'a> {
    pub properties: PageProperties,
    pub root: Node<'a>,
}

impl<'a> TryFrom<Node<'a>> for Page<'a> {
    type Error = PageError;

    fn try_from(root: Node<'a>) -> Result<Self, Self::Error> {
        let maybe_properties = if let Some(child) = &root.first_child() {
            if let NodeValue::FrontMatter(ref frontmatter_str) = child.data().value {
                let matter: Matter<YAML> = Matter::new();
                let entity: ParsedEntity<RawPageProperties> = matter.parse(frontmatter_str)?;

                entity.data
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            properties: maybe_properties.unwrap_or_default().into(),
            root,
        })
    }
}
