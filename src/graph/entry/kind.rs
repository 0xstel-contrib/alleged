use crate::error::GraphError;
use chrono::NaiveDate;
use std::path::Path;

#[derive(Debug)]
pub enum EntryKind {
    Journal(NaiveDate),
    Page(String),
}

impl EntryKind {
    #[must_use]
    pub fn as_relative_path(&self) -> String {
        match self {
            Self::Journal(date) => format!("journals/{}.md", date.format("%Y_%m_%d")),
            Self::Page(title) => format!("pages/{title}.md"),
        }
    }
}

impl TryFrom<&Path> for EntryKind {
    type Error = GraphError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        for ancestor in path.ancestors() {
            if ancestor.ends_with("journals") {
                let date = NaiveDate::parse_from_str(
                    &path
                        .file_stem()
                        .ok_or(GraphError::InvalidPath(path.to_path_buf()))?
                        .to_string_lossy(),
                    "%Y_%m_%d",
                )?;
                return Ok(EntryKind::Journal(date));
            } else if ancestor.ends_with("pages") {
                return Ok(EntryKind::Page(
                    path.file_stem()
                        .ok_or(GraphError::InvalidPath(path.to_path_buf()))?
                        .to_string_lossy()
                        .into(),
                ));
            }
        }

        Err(GraphError::InvalidPath(path.to_path_buf()))
    }
}
