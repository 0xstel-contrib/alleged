use crate::block::is_block;
use comrak::Node;
use nutype::nutype;

#[nutype(validate(predicate = is_block), derive(Debug, Clone, AsRef))]
pub struct TextBlockNode<'a>(Node<'a>);
