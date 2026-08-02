use crate::{
    consts::{DEADLINE_DELIM, SCHEDULED_DELIM},
    error::ParseDueError,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DueKind {
    Scheduled,
    Deadline,
}

impl fmt::Display for DueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scheduled => write!(f, "{SCHEDULED_DELIM}"),
            Self::Deadline => write!(f, "{DEADLINE_DELIM}"),
        }
    }
}

impl FromStr for DueKind {
    type Err = ParseDueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SCHEDULED" => Ok(Self::Scheduled),
            "DEADLINE" => Ok(Self::Deadline),
            _ => Err(ParseDueError::InvalidInput),
        }
    }
}
