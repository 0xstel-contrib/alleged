use crate::error::TaskPriorityError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskPriority {
    A,
    B,
    C,
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
