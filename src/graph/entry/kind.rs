use crate::{consts::JOURNAL_FORMAT, error::GraphError};
use std::path::Path;
use time::Date;

#[derive(Debug)]
pub enum EntryKind {
    Journal(Date),
    Page(String),
}

impl EntryKind {
    #[must_use]
    pub fn as_relative_path(&self) -> String {
        match self {
            // NOTE: `JOURNAL_FORMAT` is guaranteed valid @ compile time, so **this will never panic**.
            #[allow(clippy::unwrap_used)]
            Self::Journal(date) => format!("journals/{}.md", date.format(JOURNAL_FORMAT).unwrap()),
            Self::Page(title) => format!("pages/{title}.md"),
        }
    }
}

impl TryFrom<&Path> for EntryKind {
    type Error = GraphError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        for ancestor in path.ancestors() {
            if ancestor.ends_with("journals") {
                let date = Date::parse(
                    &path
                        .file_stem()
                        .ok_or_else(|| GraphError::InvalidPath(path.to_path_buf()))?
                        .to_string_lossy(),
                    JOURNAL_FORMAT,
                )?;
                return Ok(Self::Journal(date));
            } else if ancestor.ends_with("pages") {
                return Ok(Self::Page(
                    path.file_stem()
                        .ok_or_else(|| GraphError::InvalidPath(path.to_path_buf()))?
                        .to_string_lossy()
                        .into(),
                ));
            }
        }

        Err(GraphError::InvalidPath(path.to_path_buf()))
    }
}
