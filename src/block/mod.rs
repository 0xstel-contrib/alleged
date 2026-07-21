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

#[derive(Debug, Clone)]
pub enum Block<'a> {
    Text(Text<'a>, usize),
    Task(Task<'a>, usize),
}

impl<'a> From<Task<'a>> for Block<'a> {
    fn from(task: Task<'a>) -> Self {
        let depth = task.inner.depth();
        Self::Task(task, depth)
    }
}

impl<'a> From<Text<'a>> for Block<'a> {
    fn from(text: Text<'a>) -> Self {
        let depth = text.inner.depth();
        Self::Text(text, depth)
    }
}

impl<'a> From<TextBlockNode<'a>> for Block<'a> {
    fn from(inner: TextBlockNode<'a>) -> Self {
        let depth = inner.depth();
        Self::Text(Text::from(inner), depth)
    }
}

impl<'a> From<TaskBlockNode<'a>> for Block<'a> {
    fn from(inner: TaskBlockNode<'a>) -> Self {
        let depth = inner.depth();
        Self::Task(Task::from(inner), depth)
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
