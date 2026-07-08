use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct RawPageProperties {
    icon: Option<String>,
    title: Option<String>,
    tags: Option<String>,
    template: Option<String>,
    #[serde(rename = "template-including-parent")]
    template_including_parent: Option<bool>,
    alias: Option<String>,
    filters: Option<String>,
    public: Option<bool>,
    #[serde(rename = "exclude-from-graph-view")]
    exclude_from_graph_view: Option<bool>,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct PageProperties {
    pub icon: Option<String>,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub template: Option<String>,
    pub template_including_parent: bool,
    pub alias: Vec<String>,
    pub filters: Vec<String>,
    pub public: bool,
    pub exclude_from_graph_view: bool,
}

impl From<RawPageProperties> for PageProperties {
    fn from(value: RawPageProperties) -> Self {
        // TODO: Handle tags
        let tags = Vec::new();
        let alias = value
            .alias
            .map(|c| c.split(',').map(String::from).collect())
            .unwrap_or_default();
        // TODO: Handle filters
        let filters = Vec::new();

        Self {
            tags,
            alias,
            filters,
            icon: value.icon,
            title: value.title,
            template: value.template,
            template_including_parent: value.template_including_parent.unwrap_or(false),
            public: value.public.unwrap_or(false),
            exclude_from_graph_view: value.exclude_from_graph_view.unwrap_or(false),
        }
    }
}
