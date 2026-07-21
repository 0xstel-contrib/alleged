mod builder;
mod entry;
pub use builder::*;
pub use entry::*;

use crate::error::Alleged;
use comrak::Options;
use std::{ffi::OsStr, fs, path::PathBuf};
use time::{Date, OffsetDateTime};
use walkdir::{DirEntry, WalkDir};

pub struct Graph {
    comrak_options: Options<'static>,
    exclude: Vec<String>,
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
    pub fn entries(&self) -> impl Iterator<Item = GraphEntry<'_>> {
        self.markdown_files()
            .filter_map(|entry| GraphEntry::new(entry.into_path(), &self.comrak_options).ok())
    }
    #[must_use]
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
    pub fn journals(&self) -> impl Iterator<Item = GraphEntry<'_>> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Journal(_)))
    }
    pub fn pages(&self) -> impl Iterator<Item = GraphEntry<'_>> {
        self.entries()
            .filter(|entry| matches!(entry.kind, EntryKind::Page(_)))
    }
    fn entry(&self, entry: &EntryKind) -> Result<GraphEntry<'_>, Alleged> {
        let relative_path: PathBuf = entry.as_relative_path().into();
        GraphEntry::new(self.root.join(relative_path), &self.comrak_options)
    }
    pub fn journal<D>(&self, date: D) -> Result<GraphEntry<'_>, Alleged>
    where
        D: Into<Date>,
    {
        self.entry(&EntryKind::Journal(date.into()))
    }
    pub fn today(&self) -> Result<GraphEntry<'_>, Alleged> {
        self.journal(OffsetDateTime::now_local()?.date())
    }
    pub fn page(&self, key: &str) -> Result<GraphEntry<'_>, Alleged> {
        for mut entry in self.entries() {
            if let Some(props) = entry.properties()
                && props.alias.iter().any(|a| a == key)
            {
                return Ok(entry);
            }
        }

        self.entry(&EntryKind::Page(key.to_string()))
    }
    pub fn save(&self, entry: &mut GraphEntry<'_>) -> Result<(), Alleged> {
        fs::write(entry.path(), entry.buffer().to_string().as_bytes())?;

        Ok(())
    }
}
