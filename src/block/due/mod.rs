mod kind;
mod repeater;
pub use kind::*;
pub use repeater::*;

use crate::{
    block::BlockPropertyImpl,
    consts::{DATE_FORMAT, DUE_DELIMS, DUE_REGEX, TIME_FORMAT},
    error::{Alleged, ParseDueError},
};
#[cfg(feature = "icalendar")]
use chrono::NaiveDate;
#[cfg(feature = "icalendar")]
use icalendar::{CalendarDateTime, DatePerhapsTime};
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

// Date data is already guaranteed valid because we have a parsed instance from `time`, so this is infallible.
#[allow(clippy::fallible_impl_from)]
#[cfg(feature = "icalendar")]
impl From<Due> for DatePerhapsTime {
    fn from(due: Due) -> Self {
        // NOTE: We already know our date data is valid because we have a parsed object from `time`, so **this will never panic.**
        #[allow(clippy::unwrap_used)]
        let date_naive = NaiveDate::from_ymd_opt(
            due.date.year(),
            due.date.month() as u32,
            due.date.day().into(),
        )
        .unwrap();

        // NOTE: We already know our date data is valid because we have a parsed object from `time`, so **this will never panic.**
        #[allow(clippy::unwrap_used)]
        due.time.map_or_else(
            || date_naive.into(),
            |time| {
                CalendarDateTime::Floating(
                    date_naive
                        .and_hms_opt(
                            time.hour().into(),
                            time.minute().into(),
                            time.second().into(),
                        )
                        .unwrap(),
                )
                .into()
            },
        )
    }
}

impl BlockPropertyImpl for Due {
    type Error = Alleged;

    fn extract_and(s: &str) -> Result<(String, Self), Self::Error> {
        let (text, maybe_due_str) = DUE_DELIMS
            .iter()
            .find_map(|d| s.find(d))
            .map_or((s, ""), |idx| s.split_at(idx));

        Ok((text.trim().to_string(), Self::from_str(maybe_due_str)?))
    }
}
