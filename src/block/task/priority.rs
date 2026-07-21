use crate::error::TaskPriorityError;
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum TaskPriority {
    A,
    B,
    C,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "[#A]"),
            Self::B => write!(f, "[#B]"),
            Self::C => write!(f, "[#C]"),
        }
    }
}

impl FromStr for TaskPriority {
    type Err = TaskPriorityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "[#A]" => Ok(Self::A),
            "[#B]" => Ok(Self::B),
            "[#C]" => Ok(Self::C),
            _ => Err(TaskPriorityError::InvalidPriority),
        }
    }
}
