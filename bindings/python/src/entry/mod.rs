mod iter;
mod kind;

use alleged_lib::{
    graph::{EntryBuffer, GraphEntry},
    properties::Properties,
};
pub use iter::*;
pub use kind::*;
use pyo3::{pyclass, pymethods};
use std::path::PathBuf;

#[pyclass(name = "GraphEntry")]
pub struct PyGraphEntry {
    pub(crate) inner: GraphEntry,
    #[pyo3(get)]
    kind: PyEntryKind,
}

impl From<GraphEntry> for PyGraphEntry {
    fn from(inner: GraphEntry) -> Self {
        let kind = inner.kind.clone();

        Self {
            kind: kind.into(),
            inner,
        }
    }
}

#[pymethods]
impl PyGraphEntry {
    fn buffer(&mut self) -> EntryBuffer {
        self.inner.buffer()
    }
    fn path(&self) -> PathBuf {
        self.inner.path()
    }
    fn properties(&mut self) -> Option<Properties> {
        self.inner.properties()
    }
}
