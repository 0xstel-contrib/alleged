use crate::{
    consts::{COMRAK_OPTIONS, DEFAULT_EXCLUDE},
    error::{Alleged, GraphBuilderError},
    graph::Graph,
};
use comrak::Options;
use std::{path::PathBuf, sync::Arc};

/// Helper struct to construct a [`Graph`] object. You only need to define `root`, everything else has defaults :)
pub struct GraphBuilder {
    comrak_options: Option<Options<'static>>,
    exclude: Vec<String>,
    root: Option<PathBuf>,
    #[cfg(feature = "id")]
    populate_ids: bool,
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self {
            comrak_options: None,
            exclude: DEFAULT_EXCLUDE.into_iter().map(String::from).collect(),
            root: None,
            #[cfg(feature = "id")]
            populate_ids: false,
        }
    }
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
    /// Whether or not to pre-populate blocks with IDs. Defaults to `true`
    #[cfg(feature = "id")]
    #[must_use]
    pub const fn populate_ids(mut self) -> Self {
        self.populate_ids = true;
        self
    }
    /// Try to build a [`crate::graph::Graph`]
    ///
    /// # Errors
    /// Fails if the root directory isn't set.
    pub fn build(self) -> Result<Graph, Alleged> {
        let root = self.root.ok_or(GraphBuilderError::UndefinedRootDirectory)?;
        let comrak_options = Arc::new(
            self.comrak_options
                .unwrap_or_else(|| COMRAK_OPTIONS.clone()),
        );
        let graph = Graph {
            comrak_options,
            root,
            exclude: self.exclude,
        };

        #[cfg(feature = "id")]
        if self.populate_ids {
            graph.populate_ids()?;
        }

        Ok(graph)
    }
}
