use crate::entry::PyGraphEntry;
use pyo3::{PyRef, PyRefMut, pyclass, pymethods};
use std::vec::IntoIter;

#[pyclass]
pub struct PyGraphEntryIter {
    native: IntoIter<PyGraphEntry>,
}

#[pymethods]
impl PyGraphEntryIter {
    const fn __iter__(this: PyRef<'_, Self>) -> PyRef<'_, Self> {
        this
    }
    fn __next__(mut this: PyRefMut<'_, Self>) -> Option<PyGraphEntry> {
        this.native.next()
    }
}

impl<T> From<T> for PyGraphEntryIter
where
    T: Iterator<Item = PyGraphEntry>,
{
    fn from(iter: T) -> Self {
        Self {
            native: iter.collect::<Vec<_>>().into_iter(),
        }
    }
}
