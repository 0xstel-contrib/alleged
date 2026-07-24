use crate::{block::BlockPropertyImpl, consts::PROPERTY_REGEX};
use rustc_hash::FxHashMap;
use std::convert::Infallible;
#[cfg(feature = "id")]
use uuid::Uuid;

#[allow(clippy::redundant_pub_crate)]
#[cfg(feature = "id")]
pub(crate) fn new_id_property() -> String {
    format!("id:: {}", Uuid::new_v4())
}

#[derive(Debug)]
pub struct BlockProperties(pub FxHashMap<String, String>);

impl BlockPropertyImpl for BlockProperties {
    type Error = Infallible;

    fn extract_and(s: &str) -> Result<(String, Self), Self::Error> {
        let mut properties_map = FxHashMap::default();
        let mut lines: Vec<&str> = s.lines().collect();

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

        Ok((lines.join("\n"), Self(properties_map)))
    }
}
