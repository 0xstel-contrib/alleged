use comrak::{
    Node,
    nodes::{AstNode, NodeValue},
};

fn extract_text<'a>(node: &'a AstNode<'a>, text: &mut String) {
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

pub struct Block<'a> {
    inner: Node<'a>,
    pub depth: usize,
}

impl<'a> From<Node<'a>> for Block<'a> {
    fn from(node: Node<'a>) -> Self {
        let mut depth: usize = 0;
        let mut ancestor = node.parent();

        while let Some(parent_node) = ancestor {
            if let NodeValue::Item(_) = &parent_node.data.borrow().value {
                depth += 1;
            }
            ancestor = parent_node.parent();
        }

        Self {
            inner: node,
            depth: depth.saturating_sub(1),
        }
    }
}

impl Block<'_> {
    #[must_use]
    pub fn plaintext(&self) -> String {
        let mut text = String::new();
        extract_text(self.inner, &mut text);
        text
    }
    fn edit_text_recursive<F>(node: Node<'_>, callback: &mut F)
    where
        F: FnMut(String) -> String,
    {
        for child in node
            .children()
            .filter(|c| !matches!(c.data().value, NodeValue::Item(_)))
        {
            if let NodeValue::Text(ref mut text) = child.data_mut().value {
                *text = callback(text.to_string()).into();
            }

            Self::edit_text_recursive(child, callback);
        }
    }
    pub fn edit_text<F>(&self, callback: &mut F)
    where
        F: FnMut(String) -> String,
    {
        Self::edit_text_recursive(self.inner, callback);
    }
}
