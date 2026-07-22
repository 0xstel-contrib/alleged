use crate::entry::{PyGraphEntry, PyGraphEntryIter};
use alleged_lib::{error::Alleged, graph::Graph};
use pyo3::{
    Bound, PyResult,
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyDate, PyDateAccess},
};
use std::path::PathBuf;
use time::{Date, Month};

#[pyclass(name = "Graph", frozen)]
pub struct PyGraph {
    pub(crate) inner: Graph,
    #[pyo3(get)]
    root: PathBuf,
}

impl From<Graph> for PyGraph {
    fn from(inner: Graph) -> Self {
        let root = inner.root.clone();
        Self { inner, root }
    }
}

#[pymethods]
impl PyGraph {
    #[new]
    #[pyo3(signature = (root, exclude=vec!["logseq".to_string(), "contents.md".to_string()]))]
    fn new(root: PathBuf, exclude: Vec<String>) -> Result<Self, Alleged> {
        Ok(Graph::builder().root(root).exclude(exclude).build()?.into())
    }
    fn entries(&self) -> PyGraphEntryIter {
        self.inner.entries().map(PyGraphEntry::from).into()
    }
    fn journals(&self) -> PyGraphEntryIter {
        self.inner.journals().map(PyGraphEntry::from).into()
    }
    fn pages(&self) -> PyGraphEntryIter {
        self.inner.pages().map(PyGraphEntry::from).into()
    }
    fn journal(&self, python_date: &Bound<'_, PyDate>) -> PyResult<PyGraphEntry> {
        let year = python_date.get_year();
        let month = Month::try_from(python_date.get_month())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let day = python_date.get_day();

        let date = Date::from_calendar_date(year, month, day)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(self.inner.journal(date).map(PyGraphEntry::from)?)
    }
    fn today(&self) -> Result<PyGraphEntry, Alleged> {
        self.inner.today().map(PyGraphEntry::from)
    }
    fn page(&self, key: &str) -> Result<PyGraphEntry, Alleged> {
        self.inner.page(key).map(PyGraphEntry::from)
    }
    fn save(&self, entry: &mut PyGraphEntry) -> Result<(), Alleged> {
        self.inner.save(&mut entry.inner)
    }
}
