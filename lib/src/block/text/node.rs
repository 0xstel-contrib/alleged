use crate::block::is_block;
use comrak::Node;
use nutype::nutype;

#[nutype(validate(predicate = is_block), derive(Debug, Clone, AsRef))]
/// A validated [`Node`], guaranteed to be a [`comrak::nodes::NodeValue::Paragraph`]
pub struct TextBlockNode<'a>(Node<'a>);
