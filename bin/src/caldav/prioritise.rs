use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum SyncPrioritise {
    Logseq,
    CalDav,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSyncPrioritiseError;

impl fmt::Display for ParseSyncPrioritiseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid sync prioritise option: expected one of 'logseq', 'caldav'"
        )
    }
}

impl FromStr for SyncPrioritise {
    type Err = ParseSyncPrioritiseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "logseq" => Ok(Self::Logseq),
            "caldav" => Ok(Self::CalDav),
            _ => Err(ParseSyncPrioritiseError),
        }
    }
}
