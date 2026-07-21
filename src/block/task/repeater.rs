use crate::error::ParseRepeaterErr;
use humantime::{Duration as HumanDuration, format_duration};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr, time::Duration};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RepeatFrom {
    // ".+1d"
    Completion,
    // "+1d"
    PrevScheduled,
    // "++1d"
    PrevScheduledConstrained,
}

impl fmt::Display for RepeatFrom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completion => write!(f, ".+"),
            Self::PrevScheduled => write!(f, "+"),
            Self::PrevScheduledConstrained => write!(f, "++"),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScheduledRepeater {
    pub rule: RepeatFrom,
    pub duration: Duration,
}

impl fmt::Display for ScheduledRepeater {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rule, format_duration(self.duration))
    }
}

impl FromStr for ScheduledRepeater {
    type Err = ParseRepeaterErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let maybe_repeater = match chars.next().ok_or(ParseRepeaterErr)? {
            '.' => {
                if chars.next() == Some('+') {
                    Ok((RepeatFrom::Completion, chars))
                } else {
                    Err(ParseRepeaterErr)
                }
            }
            '+' => {
                if chars.as_str().starts_with('+') {
                    _ = chars.next();
                    Ok((RepeatFrom::PrevScheduledConstrained, chars))
                } else {
                    Ok((RepeatFrom::PrevScheduled, chars))
                }
            }
            _ => Err(ParseRepeaterErr),
        };

        let (rule, duration_chars) = maybe_repeater?;
        // HACK: Logseq's `m` means "minute" -- to `humantime`, `m` is month. We manually fix that :)
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
