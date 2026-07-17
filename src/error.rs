use chrono::ParseError;
use std::{fmt, io, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphBuilderError {
    #[error("Root directory wasn't defined!")]
    UndefinedRootDirectory,
    #[error("File read failed! ({0})")]
    FileReadFailed(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Chrono couldn't parse the string: {0}")]
    Chrono(#[from] ParseError),
    #[error("Invalid Logseq graph entry path: {0}")]
    InvalidPath(PathBuf),
    #[error("Got an I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("Failed to parse YAML: {0}")]
    GrayMatter(#[from] gray_matter::Error),
    #[error("Text formatting failed: {0}")]
    Fmt(#[from] fmt::Error),
}
