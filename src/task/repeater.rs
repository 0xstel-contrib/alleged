use crate::error::ParseRepeaterErr;
use humantime::Duration as HumanDuration;
use std::{str::FromStr, time::Duration};

#[derive(Debug)]
pub enum RepeatFrom {
    // ".+1d"
    Completion,
    // "+1d"
    PrevScheduled,
    // "++1d"
    PrevScheduledConstrained,
}

#[derive(Debug)]
pub struct ScheduledRepeater {
    pub rule: RepeatFrom,
    pub duration: Duration,
}

impl FromStr for ScheduledRepeater {
    type Err = ParseRepeaterErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let maybe_repeater = match chars.next().ok_or(ParseRepeaterErr)? {
            '.' => {
                if let Some('+') = chars.next() {
                    Ok((RepeatFrom::Completion, chars))
                } else {
                    Err(ParseRepeaterErr)
                }
            }
            '+' => {
                if let Some('+') = chars.as_str().chars().next() {
                    _ = chars.next();
                    Ok((RepeatFrom::PrevScheduledConstrained, chars))
                } else {
                    Ok((RepeatFrom::PrevScheduled, chars))
                }
            }
            _ => Err(ParseRepeaterErr),
        };

        let (rule, duration_chars) = maybe_repeater?;
        let duration_str: String = duration_chars
            .map(|c| if c == 'm' { 'M' } else { c })
            .collect();
        let duration_human: HumanDuration = duration_str.parse().map_err(|_| ParseRepeaterErr)?;

        Ok(Self {
            duration: duration_human.into(),
            rule,
        })
    }
}
