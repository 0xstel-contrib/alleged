use crate::{
    block::ScheduledRepeater,
    consts::{DATE_FORMAT, SCHEDULED_DELIM, SCHEDULED_REGEX, TIME_FORMAT},
    error::ParseScheduledError,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use time::{Date, Time, Weekday, error::InvalidVariant};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Scheduled {
    pub date: Date,
    pub day: Weekday,
    pub time: Option<Time>,
    pub repeater: Option<ScheduledRepeater>,
}

impl fmt::Display for Scheduled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: `DATE_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
        #[allow(clippy::unwrap_used)]
        write!(
            f,
            "{} <{} {}",
            SCHEDULED_DELIM,
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

impl FromStr for Scheduled {
    type Err = ParseScheduledError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = SCHEDULED_REGEX.captures(s) {
            let date_str = captures
                .get(1)
                .map(|m| m.as_str())
                .ok_or(ParseScheduledError::Generic)?;
            let day_str = captures
                .get(2)
                .map(|m| m.as_str())
                .ok_or(ParseScheduledError::Generic)?;
            let time = captures
                .get(3)
                .map(|m| m.as_str())
                .and_then(|t| Time::parse(t, TIME_FORMAT).ok());
            let repeater = captures
                .get(4)
                .map(|m| m.as_str())
                .and_then(|r| ScheduledRepeater::from_str(r).ok());

            Ok(Self {
                date: Date::parse(date_str, DATE_FORMAT)?,
                day: custom_parse_weekday(day_str)?,
                time,
                repeater,
            })
        } else {
            Err(ParseScheduledError::Generic)
        }
    }
}
