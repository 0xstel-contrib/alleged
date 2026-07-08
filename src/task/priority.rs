use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskPriority {
    A,
    B,
    C,
}

#[derive(Error, Debug)]
pub enum TaskPriorityError {
    #[error("Invalid task priority!")]
    InvalidPriority,
}

impl FromStr for TaskPriority {
    type Err = TaskPriorityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "[#A]" => Ok(TaskPriority::A),
            "[#B]" => Ok(TaskPriority::B),
            "[#C]" => Ok(TaskPriority::C),
            _ => Err(TaskPriorityError::InvalidPriority),
        }
    }
}
