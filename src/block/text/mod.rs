mod node;
use crate::block::{BlockImpl, extract_text};
use comrak::nodes::NodeValue;
pub use node::*;

/// A Logseq block with normal text content.
#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub(crate) inner: TextBlockNode<'a>,
}

impl<'a> From<TextBlockNode<'a>> for Text<'a> {
    fn from(inner: TextBlockNode<'a>) -> Self {
        Self { inner }
    }
}

impl BlockImpl for Text<'_> {
    fn raw(&self) -> String {
        let mut text = String::new();
        extract_text(self.inner.as_ref(), &mut text);
        text
    }
}

impl Text<'_> {
    /// Edit the Logseq block with the given callback.
    pub fn edit<F>(&self, callback: &mut F)
    where
        F: FnMut(&str) -> String,
    {
        for node in self.inner.as_ref().descendants() {
            if let NodeValue::Text(ref mut text) = node.data_mut().value {
                *text = callback(text).into();
            }
        }
    }
}
