#[cfg(feature = "python")]
use pyo3::{pyclass, pymethods};
use rustc_hash::FxHashMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// The properties of a page in your Logseq graph. See <https://docs.logseq.com/#/page/properties>
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass(from_py_object, get_all))]
pub struct BufferProperties {
    /// Icon identifier
    pub icon: Option<String>,
    /// Overrides page title and allows it to be different from the filename
    pub title: Option<String>,
    /// A list of wikilinks or normal tags for a page. Not really implemented yet, we just split the variable by commas (which isn't how it's supposed to actually be parsed...)
    pub tags: Vec<String>,
    /// Desginates a page/block as a template
    pub template: Option<String>,
    /// Block-specific indicator and not applicable to pages, I think -- so TBA
    pub template_including_parent: bool,
    /// A list of comma-delimited alternate page titles. Used internally by the [`crate::graph::Graph::page`] accessor function
    pub alias: Vec<String>,
    /// A list of comma-delimited "filters?" Not sure; functionality TBA
    pub filters: Vec<String>,
    /// Whether or not this page should be included as an export
    pub public: bool,
    /// Whether or not this page should be hidden from the global graph view
    pub exclude_from_graph_view: bool,
    /// Custom page properties (i.e. anything that _isn't_ one of the above)
    pub custom: FxHashMap<String, String>,
}

impl fmt::Display for BufferProperties {
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

#[cfg_attr(feature = "python", pymethods)]
impl BufferProperties {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}
