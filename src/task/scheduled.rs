use crate::task::ScheduledRepeater;
use chrono::{NaiveDate, NaiveTime, Weekday};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::LazyLock};

pub static SCHEDULED_DELIM: &str = "SCHEDULED:";
pub static SCHEDULED_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"SCHEDULED:\s*<(\d{4}-\d{2}-\d{2})\s+([A-Za-z]{3})(?:\s+(\d{1,2}:\d{2}))?(?:\s+([\.\+]*\+\d+[ymwdh]))?>$"
    ).unwrap()
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Scheduled {
    pub date: NaiveDate,
    pub day: Weekday,
    pub time: Option<NaiveTime>,
    pub repeater: Option<ScheduledRepeater>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseScheduledError;

impl FromStr for Scheduled {
    type Err = ParseScheduledError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = SCHEDULED_REGEX.captures(s) {
            let date_str = captures
                .get(1)
                .map(|m| m.as_str())
                .ok_or(ParseScheduledError)?;
            let day_str = captures
                .get(2)
                .map(|m| m.as_str())
                .ok_or(ParseScheduledError)?;
            let time = captures
                .get(3)
                .map(|m| m.as_str())
                .and_then(|t| NaiveTime::parse_from_str(t, "%H:%M").ok());
            let repeater = captures
                .get(4)
                .map(|m| m.as_str())
                .and_then(|r| ScheduledRepeater::from_str(r).ok());

            Ok(Self {
                date: NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|_| ParseScheduledError)?,
                day: day_str.parse().map_err(|_| ParseScheduledError)?,
                time,
                repeater,
            })
        } else {
            Err(ParseScheduledError)
        }
    }
}
