mod kind;
mod repeater;
pub use kind::*;
pub use repeater::*;

use crate::{
    consts::{DATE_FORMAT, DUE_DELIMS, DUE_REGEX, TIME_FORMAT},
    error::{Alleged, ParseDueError},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use time::{Date, Time, Weekday, error::InvalidVariant};

/// Representation of a `Due` Logseq string property. See [the official Logseq documentation](https://docs.logseq.com/#/page/tasks?anchor=ls-block-6a0878b3-8530-43f4-8ef6-268a31b39879)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Due {
    pub date: Date,
    pub day: Weekday,
    pub time: Option<Time>,
    pub repeater: Option<DueRepeater>,
    pub kind: DueKind,
}

impl fmt::Display for Due {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: `DATE_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
        #[allow(clippy::unwrap_used)]
        write!(
            f,
            "{} <{} {}",
            self.kind,
            self.date.format(DATE_FORMAT).unwrap(),
            self.day
        )?;

        if let Some(time) = &self.time {
            // NOTE: `TIME_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
            #[allow(clippy::unwrap_used)]
            write!(f, " {}", time.format(TIME_FORMAT).unwrap())?;
        }

        if let Some(repeater) = &self.repeater {
            write!(f, " {repeater}")?;
        }

        write!(f, ">")
    }
}

// HACK: `time`'s `Weekday::from_str` doesn't support shortened weekdays 😮‍💨
fn custom_parse_weekday(s: &str) -> Result<Weekday, InvalidVariant> {
    match s.trim().to_lowercase().as_str() {
        "mon" | "monday" => Ok(Weekday::Monday),
        "tue" | "tuesday" => Ok(Weekday::Tuesday),
        "wed" | "wednesday" => Ok(Weekday::Wednesday),
        "thu" | "thursday" => Ok(Weekday::Thursday),
        "fri" | "friday" => Ok(Weekday::Friday),
        "sat" | "saturday" => Ok(Weekday::Saturday),
        "sun" | "sunday" => Ok(Weekday::Sunday),
        _ => Err(InvalidVariant),
    }
}

impl FromStr for Due {
    type Err = Alleged;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = DUE_REGEX.captures(s) {
            let kind = captures
                .get(1)
                .ok_or(ParseDueError::InvalidInput)
                .and_then(|m| DueKind::from_str(m.as_str()))?;
            let date = captures
                .get(2)
                .ok_or(Alleged::ParseScheduled(ParseDueError::InvalidInput))
                .and_then(|m| Date::parse(m.as_str(), DATE_FORMAT).map_err(Alleged::from))?;
            let day = captures
                .get(3)
                .ok_or(Alleged::ParseScheduled(ParseDueError::InvalidInput))
                .and_then(|m| custom_parse_weekday(m.as_str()).map_err(Alleged::from))?;
            let time = captures
                .get(4)
                .map(|m| m.as_str())
                .and_then(|t| Time::parse(t, TIME_FORMAT).ok());
            let repeater = captures
                .get(5)
                .map(|m| m.as_str())
                .and_then(|r| DueRepeater::from_str(r).ok());

            return Ok(Self {
                date,
                day,
                time,
                repeater,
                kind,
            });
        }

        Err(ParseDueError::InvalidInput.into())
    }
}

impl Due {
    pub(crate) fn extract_and(s: &str) -> Result<(String, Self), Alleged> {
        let (text, maybe_due_str) = DUE_DELIMS
            .iter()
            .find_map(|d| s.find(d))
            .map_or((s, ""), |idx| s.split_at(idx));

        Ok((text.trim().to_string(), Self::from_str(maybe_due_str)?))
    }
}
