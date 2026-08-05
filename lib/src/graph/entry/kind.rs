use crate::{
    consts::JOURNAL_FORMAT,
    error::{Alleged, EntryError},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::path::Path;
use time::Date;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// The kind of Logseq graph entry
pub enum EntryKind {
    /// A journal entry with an underlying [`Date`]
    Journal(Date),
    /// A page with a title
    Page(String),
}

impl EntryKind {
    #[must_use]
    /// Turn the entry kind into a path relative to the graph root. Produces "journals/{YYYY-MM-DD}.md" for journals and "pages/{title}.md" for pages
    ///
    /// # Panics
    /// This function will never panic.
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
    type Error = Alleged;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        for ancestor in path.ancestors() {
            if ancestor.ends_with("journals") {
                let date = Date::parse(
                    &path
                        .file_stem()
                        .ok_or_else(|| EntryError::InvalidPath(path.to_path_buf()))?
                        .to_string_lossy(),
                    JOURNAL_FORMAT,
                )?;
                return Ok(Self::Journal(date));
            } else if ancestor.ends_with("pages") {
                return Ok(Self::Page(
                    path.file_stem()
                        .ok_or_else(|| EntryError::InvalidPath(path.to_path_buf()))?
                        .to_string_lossy()
                        .into(),
                ));
            }
        }

        Err(EntryError::InvalidPath(path.to_path_buf()).into())
    }
}
