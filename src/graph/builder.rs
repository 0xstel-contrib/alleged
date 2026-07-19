use crate::{consts::COMRAK_OPTIONS, error::GraphBuilderError, graph::Graph};
use comrak::Options;
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
        let comrak_options = self.comrak_options.unwrap_or(COMRAK_OPTIONS.clone());

        Ok(Graph {
            comrak_options,
            root,
            exclude: self.exclude,
        })
    }
}
