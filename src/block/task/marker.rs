use crate::error::TaskMarkerError;
use std::{fmt, str::FromStr};

#[derive(Debug)]
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

impl fmt::Display for TaskMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToDo => write!(f, "TODO"),
            Self::Doing => write!(f, "DOING"),
            Self::Done => write!(f, "DONE"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Waiting => write!(f, "WAITING"),
        }
    }
}
