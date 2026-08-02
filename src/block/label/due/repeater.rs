use crate::error::{Alleged, ParseRepeaterErr};
use humantime::{Duration as HumanDuration, format_duration};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr, time::Duration};

/// A Logseq `SCHEDULED` repeater rule type. See [the official Logseq documentation](https://docs.logseq.com/#/page/tasks?anchor=ls-block-6a0878b3-8530-43f4-8ef6-268a31b39879)
#[derive(Debug, Clone)]
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

/// A Logseq `SCHEDULED` repeater rule. See [the official Logseq documentation](https://docs.logseq.com/#/page/tasks?anchor=ls-block-6a0878b3-8530-43f4-8ef6-268a31b39879)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DueRepeater {
    pub rule: RepeatFrom,
    pub duration: Duration,
}

impl fmt::Display for DueRepeater {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rule, format_duration(self.duration))
    }
}

impl FromStr for DueRepeater {
    type Err = Alleged;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let maybe_repeater = match chars.next().ok_or(ParseRepeaterErr::InvalidRepeater)? {
            '.' => {
                if chars.next() == Some('+') {
                    Ok((RepeatFrom::Completion, chars))
                } else {
                    Err(ParseRepeaterErr::InvalidRepeater)
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
            _ => Err(ParseRepeaterErr::InvalidRepeater),
        };

        let (rule, duration_chars) = maybe_repeater?;
        // HACK: Logseq's `m` means "minute" -- to `humantime`, `m` is month. We manually fix that :)
        let duration_str: String = duration_chars
            .map(|c| if c == 'm' { 'M' } else { c })
            .collect();
        let duration_human: HumanDuration = duration_str.parse()?;

        Ok(Self {
            duration: duration_human.into(),
            rule,
        })
    }
}
