mod builder;
mod error;
pub use builder::*;
pub use error::*;

use crate::{
    file::{File, FileError},
    page::{Page, preprocess_logseq_markdown},
};
use comrak::{Node, Options, nodes::NodeValue, parse_document};
use rustc_hash::FxHashMap;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub struct Graph {
    comrak_options: Options<'static>,
    exclude: Vec<String>,
    pub root: PathBuf,
    pub buffers: FxHashMap<PathBuf, String>,
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

    fn markdown_files<'a>(&self) -> impl Iterator<Item = File<'a>> {
        WalkDir::new(&self.root)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !self.is_excluded(e))
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| entry.path().extension() == Some(OsStr::new("md")))
            .map(File::from)
    }

    pub fn pages<'a>(&self) -> impl Iterator<Item = File<'a>> {
        self.markdown_files()
            .filter(|f| f.path.parent().is_some_and(|p| p.ends_with("pages")))
    }

    pub fn journals<'a>(&self) -> impl Iterator<Item = File<'a>> {
        self.markdown_files()
            .filter(|f| f.path.parent().is_some_and(|p| p.ends_with("journals")))
    }

    pub fn entries<'a>(&self) -> impl Iterator<Item = File<'a>> {
        self.pages().chain(self.journals())
    }

    fn _edit_node<'a, F>(&self, node: Node<'a>, edit_callback: &mut F)
    where
        F: FnMut(String) -> String,
    {
        for child in node.children() {
            if let NodeValue::Text(ref mut text) = child.data_mut().value {
                *text = edit_callback(text.to_string()).into();
            }

            self._edit_node(child, edit_callback);
        }
    }

    pub fn edit_node<'a, F>(&self, node: Node<'a>, edit_callback: &mut F)
    where
        F: FnMut(String) -> String,
    {
        self._edit_node(node, edit_callback);
    }

    pub fn parse_file<'a>(&self, file: &'a File<'a>) -> Result<Page<'a>, FileError> {
        let buffer = file.get_buffer()?;
        let root = parse_document(
            &file.arena,
            preprocess_logseq_markdown(buffer).as_str(),
            &self.comrak_options,
        );
        Ok(Page::try_from(root)?)
    }

    pub fn save_to_disk<'a, P>(&self, path: &P, page: &Page<'a>) -> Result<(), FileError>
    where
        P: AsRef<Path>,
    {
        fs::write(path, page.to_logseq_markdown()?.as_bytes())?;
        Ok(())
    }
}
