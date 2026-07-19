mod marker;
mod priority;
mod repeater;
mod scheduled;
pub use marker::*;
pub use priority::*;
pub use repeater::*;
pub use scheduled::*;

use crate::{consts::SCHEDULED_DELIM, error::TaskError};
use std::str::FromStr;

#[derive(Debug)]
pub struct Task {
    pub completed: bool,
    pub marker: TaskMarker,
    pub text: String,
    pub priority: Option<TaskPriority>,
    pub scheduled: Option<Scheduled>,
}

impl FromStr for Task {
    type Err = TaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scheduled = s
            .lines()
            .find_map(|line| Scheduled::from_str(line.trim()).ok());

        let mut parts = s.split(' ');

        let marker = parts.next().ok_or(TaskError::EmptyItem)?.parse()?;

        let priority = parts
            .clone()
            .next()
            .and_then(|s| s.parse().ok())
            .inspect(|_| {
                parts.next();
            });

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
