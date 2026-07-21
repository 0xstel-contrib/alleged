mod r#impl;
mod node;
mod task;
mod text;
pub use r#impl::*;
pub use node::*;
pub use task::*;
pub use text::*;

use comrak::{
    Node,
    nodes::{AstNode, NodeValue},
};

pub(crate) fn extract_text<'a>(node: &'a AstNode<'a>, text: &mut String) {
    match &node.data().value {
        NodeValue::Text(inner) => text.push_str(&inner.clone()),
        NodeValue::SoftBreak => text.push('\n'),
        _ => {
            for child in node
                .children()
                .filter(|c| !matches!(c.data().value, NodeValue::Item(_)))
            {
                extract_text(child, text);
            }
        }
    }
}

#[derive(Debug)]
pub enum Block<'a> {
    Text(Text<'a>),
    Task(Task<'a>),
}

impl<'a> From<Task<'a>> for Block<'a> {
    fn from(task: Task<'a>) -> Self {
        Self::Task(task)
    }
}

impl<'a> From<Text<'a>> for Block<'a> {
    fn from(text: Text<'a>) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<TextBlockNode<'a>> for Block<'a> {
    fn from(inner: TextBlockNode<'a>) -> Self {
        Self::Text(Text::from(inner))
    }
}

impl<'a> From<TaskBlockNode<'a>> for Block<'a> {
    fn from(inner: TaskBlockNode<'a>) -> Self {
        Self::Task(Task::from(inner))
    }
}

impl<'a> TryFrom<Node<'a>> for Block<'a> {
    type Error = TextBlockNodeError;

    fn try_from(node: Node<'a>) -> Result<Self, Self::Error> {
        if let Ok(task_node) = TaskBlockNode::try_new(node) {
            Ok(Self::from(task_node))
        } else {
            Ok(Self::from(TextBlockNode::try_new(node)?))
        }
    }
}
