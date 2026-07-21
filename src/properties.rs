use rustc_hash::FxHashMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Properties {
    pub icon: Option<String>,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub template: Option<String>,
    pub template_including_parent: bool,
    pub alias: Vec<String>,
    pub filters: Vec<String>,
    pub public: bool,
    pub exclude_from_graph_view: bool,
    pub custom: FxHashMap<String, String>,
}

impl fmt::Display for Properties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(icon) = &self.icon {
            writeln!(f, "icon:: {icon}")?;
        }
        if let Some(title) = &self.title {
            writeln!(f, "title:: {title}")?;
        }
        if !self.tags.is_empty() {
            writeln!(f, "tags:: {}", self.tags.join(", "))?;
        }
        if let Some(template) = &self.template {
            writeln!(f, "template:: {template}")?;
        }
        if self.template_including_parent {
            writeln!(f, "template-including-parent:: true")?;
        }
        if !self.alias.is_empty() {
            writeln!(f, "alias:: {}", self.alias.join(", "))?;
        }
        if !self.filters.is_empty() {
            writeln!(f, "filters:: {}", self.filters.join(", "))?;
        }
        if self.public {
            writeln!(f, "public:: true")?;
        }
        if self.exclude_from_graph_view {
            writeln!(f, "exclude-from-graph-view:: true")?;
        }

        for (key, value) in &self.custom {
            writeln!(f, "{key}:: {value}")?;
        }

        Ok(())
    }
}
