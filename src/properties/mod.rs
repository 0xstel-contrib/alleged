mod buffer;

use crate::consts::PROPERTY_REGEX;
pub use buffer::*;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Properties(pub FxHashMap<String, String>);

impl Properties {
    pub fn from_block_modify(input: &mut String) -> Self {
        let mut properties_map = FxHashMap::default();
        let mut lines: Vec<&str> = input.lines().collect();

        lines.retain(|line| {
            if let Some((_, [key, value])) =
                PROPERTY_REGEX.captures(line).map(|caps| caps.extract())
            {
                properties_map.insert(key.to_string(), value.to_string());
                false
            } else {
                true
            }
        });

        *input = lines.join("\n");

        Self(properties_map)
    }
    pub fn maybe_from_page_modify(input: &mut String) -> Option<Self> {
        let mut properties_map = FxHashMap::default();
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.peek() {
            if let Some((_, [key, value])) =
                PROPERTY_REGEX.captures(line).map(|caps| caps.extract())
            {
                properties_map.insert(key.to_string(), value.to_string());
                lines.next();
            } else {
                break;
            }
        }

        let lines: Vec<&str> = lines.filter(|l| !l.trim().is_empty()).collect();
        *input = lines.join("\n");

        if properties_map.is_empty() {
            None
        } else {
            Some(Self(properties_map))
        }
    }
}
