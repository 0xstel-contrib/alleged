use crate::{consts::COMRAK_OPTIONS, error::GraphBuilderError, graph::Graph};
use comrak::Options;
use std::path::PathBuf;

#[derive(Default)]
/// Helper struct to construct a [`Graph`] object. You only need to define `root`, everything else has defaults :)
pub struct GraphBuilder {
    comrak_options: Option<Options<'static>>,
    exclude: Vec<String>,
    root: Option<PathBuf>,
}

impl GraphBuilder {
    #[must_use]
    /// Options to pass to the underlying [`comrak`] markdown parser
    pub fn comrak_options(mut self, options: Options<'static>) -> Self {
        self.comrak_options = Some(options);
        self
    }
    #[must_use]
    /// List of file/directory names to exclude from the directory walker
    pub fn exclude(mut self, exclude: Vec<String>) -> Self {
        self.exclude = exclude;
        self
    }
    #[must_use]
    /// The root of your Logseq graph
    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = Some(root);
        self
    }
    /// Try to build a [`crate::graph::Graph`]
    ///
    /// # Errors
    /// Fails if the root directory isn't set.
    pub fn build(self) -> Result<Graph, GraphBuilderError> {
        let root = self.root.ok_or(GraphBuilderError::UndefinedRootDirectory)?;
        let comrak_options = self
            .comrak_options
            .unwrap_or_else(|| COMRAK_OPTIONS.clone());

        Ok(Graph {
            comrak_options,
            root,
            exclude: self.exclude,
        })
    }
}
