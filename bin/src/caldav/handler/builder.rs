use crate::LogseqCaldav;
use alleged_lib::graph::Graph;
use http::Uri;
use vstorage::base::Storage;

pub struct LogseqCaldavBuilder<S>
where
    S: Storage,
{
    graph: Option<Graph>,
    storage: Option<S>,
    collection: Option<Uri>,
}

impl<S> Default for LogseqCaldavBuilder<S>
where
    S: Storage,
{
    fn default() -> Self {
        Self {
            graph: None,
            storage: None,
            collection: None,
        }
    }
}

impl<S> LogseqCaldavBuilder<S>
where
    S: Storage,
{
    #[must_use]
    pub fn graph(mut self, graph: Graph) -> Self {
        self.graph = Some(graph);
        self
    }
    #[must_use]
    pub fn storage(mut self, storage: S) -> Self {
        self.storage = Some(storage);
        self
    }
    #[must_use]
    pub fn collection(mut self, collection: Uri) -> Self {
        self.collection = Some(collection);
        self
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn build(self) -> LogseqCaldav<S> {
        let graph = self.graph.unwrap();
        let storage = self.storage.unwrap();
        let collection = self.collection.unwrap();

        LogseqCaldav {
            graph,
            storage,
            collection,
        }
    }
}
