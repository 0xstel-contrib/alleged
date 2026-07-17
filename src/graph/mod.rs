mod builder;
mod entry;
pub use builder::*;
use comrak::Options;
pub use entry::*;
use std::{ffi::OsStr, path::PathBuf};
use walkdir::{DirEntry, WalkDir};

pub struct Graph {
    comrak_options: Options<'static>,
    exclude: Vec<String>,
    pub root: PathBuf,
}

impl Graph {
    #[must_use]
    pub fn builder() -> GraphBuilder {
        GraphBuilder::default()
    }
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
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension() == Some(OsStr::new("md")))
    }
    pub fn entries(&self) -> impl Iterator<Item = GraphEntry<'_>> {
        self.markdown_files()
            .filter_map(|entry| GraphEntry::new(entry.into_path(), &self.comrak_options).ok())
    }
}
