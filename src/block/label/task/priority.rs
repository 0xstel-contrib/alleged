use crate::error::TaskPriorityError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Priority of a Logseq task. See [the official Logseq documentation](https://docs.logseq.com/#/page/tasks?anchor=ls-block-6a0878b3-060f-40d8-a79a-493e50b0e807)
pub enum TaskPriority {
    /// High priority
    A,
    /// Moderate priority
    B,
    /// Low priority
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
