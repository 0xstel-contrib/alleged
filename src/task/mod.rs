mod marker;
mod priority;
mod repeater;
mod scheduled;
pub use marker::*;
pub use priority::*;
pub use repeater::*;
pub use scheduled::*;

use comrak::{
    Node,
    nodes::{AstNode, NodeValue},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub completed: bool,
    pub marker: TaskMarker,
    pub text: String,
    pub priority: Option<TaskPriority>,
    pub scheduled: Option<Scheduled>,
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("The given line was not a task!")]
    NotATask,
    #[error("The given list item was empty!")]
    EmptyItem,
    #[error("Got an error when processing the task marker: {0}")]
    TaskMarker(#[from] TaskMarkerError),
}

fn extract_text<'a>(node: &'a AstNode<'a>, buf: &mut String) {
    if let NodeValue::Text(text) = &node.data().value {
        buf.push_str(text);
    } else if let NodeValue::List(_) = &node.data().value {
        return;
    }

    for child in node.children() {
        extract_text(child, buf);
    }
}

impl<'a> TryFrom<Node<'a>> for Task {
    type Error = TaskError;

    fn try_from(root: Node<'a>) -> Result<Self, Self::Error> {
        let mut text = String::new();
        extract_text(root, &mut text);

        let scheduled = text
            .lines()
            .find_map(|line| Scheduled::from_str(line.trim()).ok());

        let mut parts = text.split(' ');

        let marker = parts.next().ok_or(TaskError::EmptyItem)?.parse()?;

        let priority = match parts.clone().next().and_then(|s| s.parse().ok()) {
            Some(priority) => {
                parts.next();
                Some(priority)
            }
            None => None,
        };

        let text = parts
            .collect::<Vec<_>>()
            .join(" ")
            .split(SCHEDULED_DELIM)
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        Ok(Self {
            completed: matches!(marker, TaskMarker::Done),
            text,
            marker,
            priority,
            scheduled,
        })
    }
}
