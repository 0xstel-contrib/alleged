mod r#impl;
mod label;
mod node;
mod text;

pub use r#impl::*;
pub use label::*;
pub use node::*;
pub use text::*;

use comrak::{
    Arena, Node,
    nodes::{AstNode, NodeValue},
};
use uuid::Uuid;

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

pub(crate) fn add_logseq_id_to<'a>(block_node: Node<'a>, arena: &'a Arena<'a>) {
    let softbreak_node = arena.alloc(AstNode::from(NodeValue::SoftBreak));
    let text_node = arena.alloc(AstNode::from(NodeValue::Text(
        format!("id:: {}", Uuid::new_v4()).into(),
    )));
    block_node.append(softbreak_node);
    block_node.append(text_node);
}

/// A Logseq block -- either text or a task. Each variant is a tuple with the underlying object and the block's depth.
#[derive(Debug, Clone)]
pub enum Block<'a> {
    Text(Text<'a>, usize),
    Task(Task<'a>, usize),
}

impl<'a> Block<'a> {
    pub(crate) fn node(&self) -> Node<'a> {
        match self {
            Self::Text(text, _) => text.inner.as_ref(),
            Self::Task(task, _) => task.inner.as_ref(),
        }
    }
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
