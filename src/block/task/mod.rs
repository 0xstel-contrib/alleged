mod marker;
mod node;
mod priority;
mod repeater;
mod scheduled;
pub use marker::*;
pub use node::*;
pub use priority::*;
pub use repeater::*;
pub use scheduled::*;

use crate::{block::BlockImpl, consts::SCHEDULED_DELIM};
use comrak::nodes::NodeValue;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub struct Task<'a> {
    pub(crate) inner: TaskBlockNode<'a>,
    pub marker: TaskMarker,
    pub text: String,
    pub priority: Option<TaskPriority>,
    pub scheduled: Option<Scheduled>,
}

impl fmt::Display for Task<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.marker)?;
        if let Some(priority) = &self.priority {
            write!(f, "{priority} ")?;
        }

        write!(f, "{}", self.text)?;

        if let Some(scheduled) = &self.scheduled {
            write!(f, "\n{scheduled}")?;
        }

        Ok(())
    }
}

#[allow(clippy::fallible_impl_from)]
impl<'a> From<TaskBlockNode<'a>> for Task<'a> {
    fn from(inner: TaskBlockNode<'a>) -> Self {
        let inner_text = inner.as_ref().collect_text();
        let mut words = inner_text.split_whitespace().peekable();

        // NOTE: `TaskBlockNode` is a newtype which validates that the first word
        // NOTE: is a task marker, so **this will never panic**.
        #[allow(clippy::unwrap_used)]
        let marker = TaskMarker::from_str(words.next().unwrap()).unwrap();

        let priority = words.peek().and_then(|w| w.parse().ok()).inspect(|_| {
            words.next();
        });

        let text = words.collect::<Vec<_>>().join(" ");
        let mut text_parts = text.splitn(2, SCHEDULED_DELIM);

        Self {
            text: text_parts.next().unwrap_or("").trim().to_string(),
            scheduled: text_parts
                .next()
                .and_then(|part| Scheduled::from_str(part).ok()),
            inner,
            marker,
            priority,
        }
    }
}

impl BlockImpl for Task<'_> {
    fn raw(&self) -> String {
        self.to_string()
    }
    fn plain(&self) -> String {
        self.text.clone()
    }
}

impl Task<'_> {
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
