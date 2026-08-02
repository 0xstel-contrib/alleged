mod builder;
mod entry;
pub use builder::*;
pub use entry::*;

use crate::error::Alleged;
use comrak::Options;
use std::{ffi::OsStr, fs, path::PathBuf, sync::Arc};
use time::{Date, OffsetDateTime};
use walkdir::{DirEntry, WalkDir};

/// Representation of a Logseq graph
pub struct Graph {
    comrak_options: Arc<Options<'static>>,
    exclude: Vec<String>,
    /// The path to your Logseq graph root -- i.e., a folder with the following subdirectories:
    /// - `journals/`
    /// - `logseq/`
    /// - `pages/`
    pub root: PathBuf,
}

impl Graph {
    fn is_excluded(&self, entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .is_some_and(|name| self.exclude.contains(&name.to_string()))
    }
    fn markdown_files(&self) -> impl Iterator<Item = DirEntry> {
        WalkDir::new(&self.root)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e))
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("md"))
            })
    }
    /// Create an instance of [`Graph`] with the builder pattern
    #[must_use]
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
    /// All markdown files in the Logseq graph directory ([`Graph::root`])
    pub fn entries(&self) -> impl Iterator<Item = GraphEntry> {
        self.markdown_files()
            .filter_map(|entry| GraphEntry::new(entry.into_path(), &self.comrak_options).ok())
    }
    /// All markdown files in the Logseq graph's `journals` subdirectory
    pub fn journals(&self) -> impl Iterator<Item = GraphEntry> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Journal(_)))
    }
    /// All markdown files in the Logseq graph's `pages` subdirectory
    pub fn pages(&self) -> impl Iterator<Item = GraphEntry> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Page(_)))
    }
    fn entry(&self, entry: &EntryKind) -> Result<GraphEntry, Alleged> {
        let relative_path: PathBuf = entry.as_relative_path().into();
        GraphEntry::new(self.root.join(relative_path), &self.comrak_options)
    }
    /// Get a journal entry from the given date
    ///
    /// # Errors
    /// If your graph root is valid, this should never fail.
    pub fn journal<D>(&self, date: D) -> Result<GraphEntry, Alleged>
    where
        D: Into<Date>,
    {
        self.entry(&EntryKind::Journal(date.into()))
    }
    /// Ease-of-access to today's journal entry
    ///
    /// # Errors
    /// If your graph root is valid, this should never fail.
    pub fn today(&self) -> Result<GraphEntry, Alleged> {
        self.journal(OffsetDateTime::now_local()?.date())
    }
    /// Get a page by its key. Doesn't validate whether or not an entry exists, so if you need such validation, you should probably call [`std::path::Path::exists`] on the value of [`GraphEntry::path`]
    ///
    /// # Errors
    /// If your graph root is valid, this should never fail.
    pub fn page(&self, key: &str) -> Result<GraphEntry, Alleged> {
        for mut entry in self.entries() {
            if let Some(props) = entry.properties()
                && props.alias.iter().any(|a| a == key)
            {
                return Ok(entry);
            }
        }

        self.entry(&EntryKind::Page(key.to_string()))
    }
    /// Save a Logseq graph entry to disk
    ///
    /// # Errors
    /// Throws an error if the filesystem write fails.
    pub fn save(&self, entry: &mut GraphEntry) -> Result<(), Alleged> {
        fs::write(entry.path(), entry.buffer().to_string().as_bytes())?;

        Ok(())
    }
}
