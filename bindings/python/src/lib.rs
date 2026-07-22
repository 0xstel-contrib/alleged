//! Python bindings for the [`alleged-lib`] crate

mod block;
mod entry;
mod graph;

use pyo3::pymodule;

#[doc = include_str!("../README.md")]
#[pymodule]
mod alleged {
    #[pymodule_export]
    use super::graph::PyGraph;
}
