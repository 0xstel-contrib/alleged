use crate::{consts::PROPERTY_REGEX, graph::BufferProperties};
use std::{
    convert::Infallible,
    fmt::{self, Write},
    str::FromStr,
};

#[derive(Debug, Default, Clone)]
/// Representation of a file in your Logseq graph
pub struct EntryBuffer {
    /// A file's (parsed) Logseq properties
    pub properties: Option<BufferProperties>,
    /// Everything after the properties section, if it exists -- otherwise just the entire file
    pub content: String,
}

impl FromStr for EntryBuffer {
    type Err = Infallible;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines().peekable();
        let mut maybe_properties = None;

        while let Some(line) = lines.peek() {
            if let Some((_, [key, value])) =
                PROPERTY_REGEX.captures(line).map(|caps| caps.extract())
            {
                let properties: &mut BufferProperties = maybe_properties.get_or_insert_default();
                match key {
                    "icon" => properties.icon = Some(value.to_string()),
                    "title" => properties.title = Some(value.to_string()),
                    "tags" => properties.tags = value.split(',').map(String::from).collect(),
                    "template" => properties.template = Some(value.to_string()),
                    "template_including_parent" => {
                        properties.template_including_parent =
                            value.trim().parse().unwrap_or(false);
                    }
                    "alias" => properties.alias = value.split(',').map(String::from).collect(),
                    "filters" => properties.filters = value.split(',').map(String::from).collect(),
                    "public" => properties.public = value.trim().parse().unwrap_or(false),
                    "exclude_from_graph_view" => {
                        properties.exclude_from_graph_view = value.trim().parse().unwrap_or(false);
                    }
                    _ => _ = properties.custom.insert(key.to_string(), value.to_string()),
                }
                lines.next();
            } else {
                break;
            }
        }

        let content_lines: Vec<&str> = lines.filter(|l| !l.trim().is_empty()).collect();

        Ok(Self {
            properties: maybe_properties,
            content: content_lines.join("\n"),
        })
    }
}

impl fmt::Display for EntryBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(properties) = &self.properties {
            writeln!(f, "{properties}")?;
        }

        writeln!(f, "{}", self.content)
    }
}

impl EntryBuffer {
    fn prepend_or_append_block(
        &mut self,
        content: &str,
        depth: usize,
        prepend: bool,
    ) -> fmt::Result {
        let indent = "\t".repeat(depth);
        let new_block = content
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    format!("{indent}- {line}")
                } else {
                    format!("{indent}  {line}")
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        if prepend {
            self.content.insert_str(0, &format!("{new_block}\n"));
        } else {
            write!(self.content, "\n{new_block}")?;
        }

        Ok(())
    }
    /// Prepend a text block to the underlying text content (after properties) with the specified indent
    ///
    /// # Errors
    /// Throws an error if the underlying [`write!`] call fails
    pub fn prepend_block(&mut self, content: &str, depth: usize) -> fmt::Result {
        self.prepend_or_append_block(content, depth, true)
    }
    /// Append a text block to the underlying text content with the specified intent
    ///
    /// # Errors
    /// Throws an error if the underlying [`write!`] call fails
    pub fn append_block(&mut self, content: &str, depth: usize) -> fmt::Result {
        self.prepend_or_append_block(content, depth, false)
    }
}
