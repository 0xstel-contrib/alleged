mod kind;
pub use kind::*;

use crate::{
    block::Block,
    consts::GRAPH_LAYOUT,
    error::GraphError,
    preprocess::preprocess_logseq_markdown,
    properties::{Properties, RawProperties},
};
use comrak::{Arena, Node, Options, format_commonmark, nodes::NodeValue, parse_document};
use gray_matter::{Matter, ParsedEntity, engine::YAML};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Document<'a, I>(pub Node<'a>, pub I);

#[derive(Debug)]
pub struct GraphEntry<'a> {
    pub kind: EntryKind,
    comrak_options: &'a Options<'a>,
    buffer: Option<String>,
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
    fn buffer_preprocessed<F>(&mut self, preprocessor: F) -> String
    where
        F: Fn(&str) -> String,
    {
        preprocessor(&self.buffer())
    }
    pub fn buffer(&mut self) -> String {
        let path = self.path();
        self.buffer
            .get_or_insert_with(|| fs::read_to_string(path).unwrap_or_default())
            .clone()
    }
    pub fn buffer_mut(&mut self) -> &mut String {
        let path = self.path();
        self.buffer
            .get_or_insert_with(|| fs::read_to_string(path).unwrap_or_default())
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
    pub fn properties(&mut self) -> Result<Option<Properties>, GraphError> {
        let arena = Arena::new();
        let buffer = self.buffer_preprocessed(preprocess_logseq_markdown);
        let root = parse_document(&arena, &buffer, self.comrak_options);

        let maybe_properties = if let Some(first_child) = root.first_child()
            && let NodeValue::FrontMatter(ref frontmatter_str) = first_child.data().value
        {
            let parser: Matter<YAML> = Matter::new();
            let entity: ParsedEntity<RawProperties> = parser.parse(frontmatter_str.trim())?;

            Some(entity.data.unwrap_or_default().into())
        } else {
            None
        };

        Ok(maybe_properties)
    }
    pub fn blocks<'b>(
        &mut self,
        arena: &'b Arena<'b>,
    ) -> Document<'b, impl Iterator<Item = Block<'b>>> {
        let buffer = self.buffer_preprocessed(preprocess_logseq_markdown);
        let root = parse_document(arena, &buffer, self.comrak_options);

        if let Some(first_child) = root.first_child()
            && matches!(first_child.data().value, NodeValue::FrontMatter(_))
        {
            first_child.detach();
        }

        let blocks = root
            .descendants()
            .filter_map(|node| {
                if matches!(node.data().value, NodeValue::Item(_)) {
                    node.first_child()
                } else {
                    None
                }
            })
            .map(Block::from);

        Document(root, blocks)
    }
    pub fn update_buffer(&mut self, root: Node<'_>) -> Result<String, GraphError> {
        let mut markdown = String::new();

        format_commonmark(root, self.comrak_options, &mut markdown)?;

        if let Some(properties) = self.properties()? {
            markdown.insert_str(0, &properties.to_string());
        }

        self.buffer = Some(markdown.clone());

        Ok(markdown)
    }
}
