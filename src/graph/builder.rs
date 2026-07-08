use crate::{
    consts::DEFAULT_COMRAK_OPTIONS,
    graph::{Graph, GraphBuilderError},
};
use comrak::Options;
use rustc_hash::FxHashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct GraphBuilder {
    comrak_options: Option<Options<'static>>,
    exclude: Vec<String>,
    root: Option<PathBuf>,
}

impl GraphBuilder {
    #[must_use]
    pub fn comrak_options(mut self, options: Options<'static>) -> Self {
        self.comrak_options = Some(options);
        self
    }
    #[must_use]
    pub fn exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = exclude;
        self
    }
    #[must_use]
    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = Some(root);
        self
    }
    pub fn build(self) -> Result<Graph, GraphBuilderError> {
        let root = self.root.ok_or(GraphBuilderError::UndefinedRootDirectory)?;
        let comrak_options = self
            .comrak_options
            .unwrap_or(DEFAULT_COMRAK_OPTIONS.clone());

        Ok(Graph {
            comrak_options,
            root,
            exclude: self.exclude,
            buffers: FxHashMap::default(),
        })
    }
}
