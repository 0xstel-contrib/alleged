mod buffer;
mod kind;
pub use buffer::*;
pub use kind::*;

use crate::{block::Block, consts::GRAPH_LAYOUT, error::GraphError, properties::Properties};
use comrak::{Arena, Node, Options, format_commonmark, parse_document};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct Document<'a, I>(pub Node<'a>, pub I);

#[derive(Debug)]
pub struct GraphEntry<'a> {
    pub kind: EntryKind,
    comrak_options: &'a Options<'a>,
    buffer: Option<EntryBuffer>,
    graph: PathBuf,
}

impl<'a> GraphEntry<'a> {
    fn root_from_entry_path<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
        for ancestor in path.as_ref().ancestors() {
            if GRAPH_LAYOUT
                .iter()
                .map(|dir_name| ancestor.join(dir_name))
                .all(|child| child.is_dir())
            {
                return Some(ancestor.to_path_buf());
            }
        }

        None
    }
    pub fn buffer(&mut self) -> EntryBuffer {
        let path = self.path();
        let buffer = self.buffer.get_or_insert_with(|| {
            EntryBuffer::from_str(&fs::read_to_string(path).unwrap_or_default()).unwrap_or_default()
        });
        buffer.clone()
    }
    fn buffer_mut(&mut self) -> &mut EntryBuffer {
        let path = self.path();
        self.buffer.get_or_insert_with(|| {
            EntryBuffer::from_str(&fs::read_to_string(path).unwrap_or_default()).unwrap_or_default()
        })
    }
    pub fn new(path: PathBuf, comrak_options: &'a Options<'a>) -> Result<Self, GraphError> {
        let kind = EntryKind::try_from(path.as_path())?;
        let graph =
            Self::root_from_entry_path(path.clone()).ok_or(GraphError::InvalidPath(path))?;

        Ok(Self {
            buffer: None,
            kind,
            comrak_options,
            graph,
        })
    }
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.graph.join(self.kind.as_relative_path())
    }
    pub fn properties(&mut self) -> Option<Properties> {
        self.buffer().properties
    }
    pub fn blocks<'b>(
        &mut self,
        arena: &'b Arena<'b>,
    ) -> Document<'b, impl Iterator<Item = Block<'b>>> {
        let buffer = self.buffer().content;
        let root = parse_document(arena, &buffer, self.comrak_options);

        let blocks = root
            .descendants()
            .filter_map(|node| Block::try_from(node).ok());

        Document(root, blocks)
    }
    pub fn update_buffer(&mut self, root: Node<'_>) -> Result<String, GraphError> {
        let comrak_options = self.comrak_options;
        let buffer = self.buffer_mut();

        buffer.content.clear();
        format_commonmark(root, comrak_options, &mut buffer.content)?;

        Ok(buffer.content.clone())
    }
    pub fn prepend_block(&mut self, content: &str, depth: usize) -> fmt::Result {
        self.buffer_mut().prepend_block(content, depth)
    }
    pub fn append_block(&mut self, content: &str, depth: usize) -> fmt::Result {
        self.buffer_mut().append_block(content, depth)
    }
}
