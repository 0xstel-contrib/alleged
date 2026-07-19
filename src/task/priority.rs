use crate::error::TaskPriorityError;
use std::str::FromStr;

#[derive(Debug)]
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
