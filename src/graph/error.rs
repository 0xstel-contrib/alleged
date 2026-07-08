use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphBuilderError {
    #[error("Root directory wasn't defined!")]
    UndefinedRootDirectory,
    #[error("File read failed! ({0})")]
    FileReadFailed(#[from] io::Error),
}
