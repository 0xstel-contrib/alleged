mod node;
use comrak::nodes::NodeValue;
pub use node::*;

use crate::{
    block::{BlockImpl, BlockNodeImpl, extract_text},
    consts::LOGSEQ_TOKENS,
};

#[derive(Debug)]
pub struct Text<'a> {
    inner: TextBlockNode<'a>,
    pub depth: usize,
}

impl<'a> From<TextBlockNode<'a>> for Text<'a> {
    fn from(inner: TextBlockNode<'a>) -> Self {
        Self {
            depth: inner.depth(),
            inner,
        }
    }
}

impl BlockImpl for Text<'_> {
    fn raw(&self) -> String {
        let mut text = String::new();
        extract_text(self.inner.as_ref(), &mut text);
        text
    }
    fn plain(&self) -> String {
        let raw = self.raw();

        raw.lines()
            .filter(|line| LOGSEQ_TOKENS.iter().any(|tok| !line.contains(tok)))
            .collect()
    }
}

impl Text<'_> {
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
