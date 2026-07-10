mod builder;
mod error;
pub use builder::*;
pub use error::*;

use crate::{
    file::{File, FileError},
    page::Page,
};
use comrak::{Node, Options, nodes::NodeValue, parse_document};
use rustc_hash::FxHashMap;
use std::{borrow::Cow, ffi::OsStr, fs, path::PathBuf};
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

    fn _edit_node<'a, F>(&self, node: Node<'a>, mut edit_callback: F)
    where
        F: FnMut(&mut Cow<'static, str>) -> String,
    {
        for child in node.children() {
            if let NodeValue::Text(ref mut text) = child.data_mut().value {
                *text = edit_callback(text).into();
            }

            self._edit_node(child, &mut edit_callback);
        }
    }

    pub fn edit_node<'a, F>(
        &self,
        node: Node<'a>,
        file: &'a mut File<'a>,
        page: Page<'a>,
        mut edit_callback: F,
    ) -> Result<(), FileError>
    where
        F: FnMut(&mut Cow<'static, str>) -> String,
    {
        self._edit_node(node, &mut edit_callback);
        let markdown = page.to_logseq_markdown()?;
        fs::write(&file.path, markdown.as_bytes())?;
        file.load()?;

        Ok(())
    }

    pub fn parse_file<'a, F>(
        &self,
        file: &'a File<'a>,
        preprocess_markdown: F,
    ) -> Result<Page<'a>, FileError>
    where
        F: Fn(&str) -> String,
    {
        let buffer = file.get_buffer()?;
        let root = parse_document(
            &file.arena,
            preprocess_markdown(buffer).as_str(),
            &self.comrak_options,
        );
        let mut page = Page::try_from(root)?;

        // HACK: For pages without an explicit title in their properties, override w/ file name
        page.properties.title = match page.properties.title {
            Some(_) => page.properties.title,
            None => file
                .path
                .file_stem()
                .and_then(|s| s.to_str().map(String::from)),
        };

        Ok(page)
    }
}
