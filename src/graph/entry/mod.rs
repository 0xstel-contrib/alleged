mod buffer;
mod kind;
pub use buffer::*;
pub use kind::*;

use crate::{
    block::Block,
    consts::GRAPH_LAYOUT,
    error::{Alleged, EntryError},
    properties::Properties,
};
use comrak::{Arena, Node, Options, format_commonmark, parse_document};
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

/// Basic wrapper around [`Node`] used by [`GraphEntry::blocks`] to return:
/// - the document root
/// - an iterator over that root's blocks, all of which can be mutated
pub struct Document<'a, I>(pub Node<'a>, pub I);

#[derive(Debug, Clone)]
/// A file/page/entry on a Logseq graph
pub struct GraphEntry {
    pub kind: EntryKind,
    comrak_options: Arc<Options<'static>>,
    buffer: Option<EntryBuffer>,
    graph: PathBuf,
}

impl GraphEntry {
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
    /// Immutable access to the underlying buffer. Requires `&mut self` because it sets the buffer if it's `None`
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
    /// Create an entry from the given path and markdown parser options. Doesn't read the file content! Deduces [`EntryKind`] from the path
    ///
    /// # Errors
    /// Throws an error if the given path is not a member of a valid Logseq graph.
    pub fn new(path: PathBuf, comrak_options: &Arc<Options<'static>>) -> Result<Self, Alleged> {
        let comrak_options = Arc::clone(comrak_options);
        let kind = EntryKind::try_from(path.as_path())?;
        let graph =
            Self::root_from_entry_path(path.clone()).ok_or(EntryError::InvalidPath(path))?;

        Ok(Self {
            buffer: None,
            kind,
            comrak_options,
            graph,
        })
    }
    /// Returns the entry's full path
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.graph.join(self.kind.as_relative_path())
    }
    /// Returns the page's properties
    pub fn properties(&mut self) -> Option<Properties> {
        self.buffer().properties
    }
    /// Returns a [`Document`] with an iterator over the entry's blocks. Returns a reference to the document root [`Node`], which you need to pass to [`GraphEntry::update_buffer`] if you update the blocks
    pub fn blocks<'b>(
        &mut self,
        arena: &'b Arena<'b>,
    ) -> Document<'b, impl Iterator<Item = Block<'b>>> {
        let buffer = self.buffer().content;
        let root = parse_document(arena, &buffer, self.comrak_options.clone().as_ref());

        let blocks = root
            .descendants()
            .filter_map(|node| Block::try_from(node).ok());

        Document(root, blocks)
    }
    /// Given a node, updates the entry's underlying text buffer. Lets you save any edits to blocks produced by [`GraphEntry::blocks`]
    ///
    /// # Errors
    /// Fails if the root [`Node`] contains invalid structure. Unless you're modifying it by hand, that should be impossible
    pub fn update_buffer(&mut self, root: Node<'_>) -> Result<String, Alleged> {
        let comrak_options = self.comrak_options.clone();
        let buffer = self.buffer_mut();

        buffer.content.clear();
        format_commonmark(root, comrak_options.as_ref(), &mut buffer.content)?;

        Ok(buffer.content.clone())
    }
    /// Prepend a text block to document with the specified indent
    ///
    /// # Errors
    /// Throws an error if the underlying [`write!`] call fails
    pub fn prepend_block(&mut self, content: &str, depth: usize) -> fmt::Result {
        self.buffer_mut().prepend_block(content, depth)
    }
    /// Append a text block to document with the specified indent
    ///
    /// # Errors
    /// Throws an error if the underlying [`write!`] call fails
    pub fn append_block(&mut self, content: &str, depth: usize) -> fmt::Result {
        self.buffer_mut().append_block(content, depth)
    }
}
