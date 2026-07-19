use std::{fmt, io, path::PathBuf};
use thiserror::Error;
use time::error::{InvalidVariant, Parse};

#[derive(Error, Debug)]
pub enum GraphBuilderError {
    #[error("Root directory wasn't defined!")]
    UndefinedRootDirectory,
    #[error("File read failed! ({0})")]
    FileReadFailed(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Invalid Logseq graph entry path: {0}")]
    InvalidPath(PathBuf),
    #[error("Got an I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("Text formatting failed: {0}")]
    Fmt(#[from] fmt::Error),
    #[error("Failed to parse the date str: {0}")]
    Date(#[from] Parse),
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
#[error("Failed to parse the repetition rule string!")]
pub struct ParseRepeaterErr;

#[derive(Error, Debug)]
pub enum TaskPriorityError {
    #[error("Invalid task priority!")]
    InvalidPriority,
}

#[derive(Error, Debug)]
pub enum ParseScheduledError {
    #[error("Failed to parse the `SCHEDULED` data!")]
    Generic,
    #[error("Failed to parse the date str: {0}")]
    Date(#[from] Parse),
    #[error("Failed to parse the time str: {0}")]
    Time(#[from] InvalidVariant),
}

#[derive(Error, Debug)]
pub enum EntryBufferError {}
