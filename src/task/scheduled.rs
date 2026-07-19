use crate::{consts::SCHEDULED_REGEX, error::ParseScheduledError, task::ScheduledRepeater};
use chrono::{NaiveDate, NaiveTime, Weekday};
use std::str::FromStr;

#[derive(Debug)]
pub struct Scheduled {
    pub date: NaiveDate,
    pub day: Weekday,
    pub time: Option<NaiveTime>,
    pub repeater: Option<ScheduledRepeater>,
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
                .and_then(|t| NaiveTime::parse_from_str(t, "%H:%M").ok());
            let repeater = captures
                .get(4)
                .map(|m| m.as_str())
                .and_then(|r| ScheduledRepeater::from_str(r).ok());

            Ok(Self {
                date: NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?,
                day: day_str.parse()?,
                time,
                repeater,
            })
        } else {
            Err(ParseScheduledError::Generic)
        }
    }
}
