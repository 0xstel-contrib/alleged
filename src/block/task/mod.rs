mod marker;
mod node;
mod priority;
pub use marker::*;
pub use node::*;
pub use priority::*;

use crate::block::{BlockImpl, BlockProperties, BlockPropertyImpl, Due, extract_text};
use comrak::nodes::NodeValue;
use std::{fmt, str::FromStr};

/// Representation of a task, with optional priority and scheduling. See <https://docs.logseq.com/#/page/tasks>
#[derive(Debug, Clone)]
pub struct Task<'a> {
    pub(crate) inner: TaskBlockNode<'a>,
    pub marker: TaskMarker,
    pub text: String,
    pub priority: Option<TaskPriority>,
    pub due: Option<Due>,
}

impl fmt::Display for Task<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.marker)?;
        if let Some(priority) = &self.priority {
            write!(f, "{priority} ")?;
        }

        write!(f, "{}", self.text)?;

        if let Some(due) = &self.due {
            write!(f, "\n{due}")?;
        }

        Ok(())
    }
}

#[allow(clippy::fallible_impl_from)]
impl<'a> From<TaskBlockNode<'a>> for Task<'a> {
    fn from(inner: TaskBlockNode<'a>) -> Self {
        let mut raw_text = String::new();
        extract_text(inner.as_ref(), &mut raw_text);
        let (content_text, _) = BlockProperties::extract_and(&raw_text).unwrap();
        let mut words = content_text.split_whitespace().peekable();

        // NOTE: `TaskBlockNode` is a newtype which validates that the first word
        // NOTE: is a task marker, so **this will never panic**.
        #[allow(clippy::unwrap_used)]
        let marker = TaskMarker::from_str(words.next().unwrap()).unwrap();

        let priority = words.peek().and_then(|w| w.parse().ok()).inspect(|_| {
            words.next();
        });

        let text = words.collect::<Vec<_>>().join(" ");
        let (text, due) =
            Due::extract_and(&text).map_or((text, None), |(text, due)| (text, Some(due)));

        Self {
            inner,
            marker,
            text,
            priority,
            due,
        }
    }
}

impl BlockImpl for Task<'_> {
    fn raw(&self) -> String {
        let mut text = String::new();
        extract_text(self.inner.as_ref(), &mut text);
        text
    }
    fn plain(&self) -> String {
        self.text.clone()
    }
    fn due(&self) -> Option<Due> {
        self.due.clone()
    }
}

impl Task<'_> {
    /// Update a task's marker. Returns a boolean indicating whether or not the marker in the underlying node was actually updated.
    pub fn mark(&mut self, marker: TaskMarker) -> bool {
        let mut updated_marker = false;

        for desc in self.inner.as_ref().descendants() {
            if updated_marker {
                break;
            }

            if let NodeValue::Text(ref mut text) = desc.data_mut().value {
                *text = text
                    .replacen(&self.marker.to_string(), &marker.to_string(), 1)
                    .into();
                updated_marker = true;
            }
        }

        self.marker = marker;

        updated_marker
    }
}
