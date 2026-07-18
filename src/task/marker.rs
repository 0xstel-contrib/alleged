use crate::error::TaskMarkerError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskMarker {
    ToDo,
    Doing,
    Done,
    Cancelled,
    Waiting,
}

impl FromStr for TaskMarker {
    type Err = TaskMarkerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TODO" => Ok(Self::ToDo),
            "DOING" => Ok(Self::Doing),
            "DONE" => Ok(Self::Done),
            "CANCELED" | "CANCELLED" => Ok(Self::Cancelled),
            "WAITING" => Ok(Self::Waiting),
            _ => Err(TaskMarkerError::InvalidMarker),
        }
    }
}
