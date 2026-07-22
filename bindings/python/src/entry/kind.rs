use alleged_lib::graph::EntryKind;
use pyo3::pyclass;

#[derive(Clone, PartialEq, Eq)]
#[pyclass(name = "EntryKind", frozen, from_py_object, eq, eq_int)]
pub enum PyEntryKind {
    Journal,
    Page,
}

impl From<EntryKind> for PyEntryKind {
    fn from(inner: EntryKind) -> Self {
        match inner {
            EntryKind::Journal(_) => Self::Journal,
            EntryKind::Page(_) => Self::Page,
        }
    }
}
