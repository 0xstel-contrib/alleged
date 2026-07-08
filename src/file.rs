use comrak::Arena;
use std::{fs, io, path::PathBuf};
use thiserror::Error;
use walkdir::DirEntry;

use crate::page::PageError;

pub struct File<'a> {
    pub(crate) arena: Arena<'a>,
    pub path: PathBuf,
    pub buffer: Option<String>,
}

impl From<DirEntry> for File<'_> {
    fn from(value: DirEntry) -> Self {
        Self::new(value.into_path())
    }
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File read failed! ({0})")]
    FileReadFailed(#[from] io::Error),
    #[error("File has to be loaded at this point, but wasn't!")]
    FileNotLoaded,
    #[error("Failed to parse the given file! ({0})")]
    FileParseFailed(#[from] PageError),
}

impl File<'_> {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            arena: Arena::new(),
            path: path.into(),
            buffer: None,
        }
    }

    pub fn load(&mut self) -> Result<&mut Self, FileError> {
        let content = fs::read_to_string(&self.path).map_err(FileError::FileReadFailed)?;
        self.buffer = Some(content);
        Ok(self)
    }

    #[must_use]
    pub fn unload(&mut self) -> &mut Self {
        self.buffer = None;
        self.arena = Arena::new();
        self
    }

    pub fn get_buffer(&self) -> Result<&String, FileError> {
        self.buffer.as_ref().ok_or(FileError::FileNotLoaded)
    }

    pub fn get_buffer_mut(&mut self) -> Result<&mut String, FileError> {
        self.buffer.as_mut().ok_or(FileError::FileNotLoaded)
    }
}
