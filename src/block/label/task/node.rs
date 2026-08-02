use crate::block::{TaskMarker, TextBlockNode};
use comrak::Node;
use nutype::nutype;
use std::str::FromStr;

fn is_task_block(node: Node<'_>) -> bool {
    TextBlockNode::try_new(node).is_ok_and(|text_node| {
        text_node
            .as_ref()
            .collect_text()
            .split_whitespace()
            .next()
            .is_some_and(|first_word| TaskMarker::from_str(first_word).is_ok())
    })
}

#[nutype(validate(predicate = is_task_block), derive(Debug, Clone, AsRef))]
/// A validated [`Node`], guaranteed to be a [`comrak::nodes::NodeValue::Paragraph`] AND have a valid [`TaskMarker`] as the first word of its textual content
pub struct TaskBlockNode<'a>(Node<'a>);
