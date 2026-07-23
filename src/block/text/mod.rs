mod node;
use crate::block::{BlockImpl, Due, extract_text};
use comrak::nodes::NodeValue;
pub use node::*;

#[derive(Debug, Clone)]
/// A Logseq block with normal text content.
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
    fn plain(&self) -> String {
        if let Ok((text, _)) = Due::extract_and(&self.raw()) {
            text
        } else {
            self.raw()
        }
    }
}

impl Text<'_> {
    /// Edit the Logseq block by the given callback.
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
    /// If this text block has a "due" marker (`SCHEDULED`/`DEADLINE`), parse and return it.
    #[must_use]
    pub fn due(&self) -> Option<Due> {
        Due::extract_and(&self.raw()).ok().map(|(_, due)| due)
    }
}
