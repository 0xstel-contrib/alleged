use humantime::DurationError;
#[cfg(feature = "python")]
use pyo3::{PyErr, exceptions::PyValueError};
use std::{fmt, io, path::PathBuf};
use thiserror::Error;
use time::error::{IndeterminateOffset, InvalidVariant, Parse};

#[derive(Error, Debug)]
/// The primary error type
pub enum Alleged {
    #[error("Graph-related failure: {0}")]
    Graph(#[from] GraphError),
    #[error("Graph builder failed: {0}")]
    GraphBuilder(#[from] GraphBuilderError),
    #[error("Entry-related error: {0}")]
    Entry(#[from] EntryError),
    #[error("Repeater parsing failed: {0}")]
    ParseRepeater(#[from] ParseRepeaterErr),
    #[error("Due parsing failed: {0}")]
    ParseScheduled(#[from] ParseDueError),
    #[error("Got an I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("Got a formatting error: {0}")]
    Fmt(#[from] fmt::Error),
    #[error("Date string parsing failed: {0}")]
    Date(#[from] Parse),
    #[error("Couldn't determine local date offset: {0}")]
    DateOffset(#[from] IndeterminateOffset),
    #[error("Time string parsing failed: {0}")]
    Time(#[from] InvalidVariant),
    #[error("HumanTime duration conversion failed: {0}")]
    HumanTime(#[from] DurationError),
}

#[cfg(feature = "python")]
impl From<Alleged> for PyErr {
    fn from(error: Alleged) -> Self {
        let error_str = format!("{error:?}");
        PyValueError::new_err(error_str)
    }
}

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("Invalid graph entry path: {0}")]
    InvalidPath(PathBuf),
}

#[derive(Error, Debug)]
pub enum GraphBuilderError {
    #[error("Root directory wasn't defined!")]
    UndefinedRootDirectory,
}

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Invalid Logseq graph entry path: {0}")]
    InvalidPath(PathBuf),
}

#[derive(Error, Debug)]
pub enum TaskMarkerError {
    #[error("Invalid task marker str!")]
    InvalidMarker,
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("The given line was not a task!")]
    NotATask,
    #[error("The given list item was empty!")]
    EmptyItem,
    #[error("Got an error when processing the task marker: {0}")]
    TaskMarker(#[from] TaskMarkerError),
}

#[derive(Error, Debug)]
pub enum ParseRepeaterErr {
    #[error("Invalid repeater string!")]
    InvalidRepeater,
}

#[derive(Error, Debug)]
pub enum TaskPriorityError {
    #[error("Invalid task priority!")]
    InvalidPriority,
}

#[derive(Error, Debug)]
pub enum ParseDueError {
    #[error("Invalid input!")]
    InvalidInput,
}
