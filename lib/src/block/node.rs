use comrak::{Node, nodes::NodeValue};

pub fn is_block(node: Node<'_>) -> bool {
    matches!(node.data().value, NodeValue::Paragraph)
}

/// Generic functions for any newtype containing a [`Node`]
pub trait BlockNodeImpl {
    /// The [`Node`]'s depth
    fn depth(&self) -> usize;
}

impl<'a, T> BlockNodeImpl for T
where
    T: AsRef<Node<'a>>,
{
    fn depth(&self) -> usize {
        let node = self.as_ref();

        let mut depth: usize = 0;
        let mut ancestor = node.parent();

        while let Some(parent_node) = ancestor {
            if let NodeValue::Item(_) = &parent_node.data().value {
                depth += 1;
            }
            ancestor = parent_node.parent();
        }

        depth.saturating_sub(1)
    }
}
