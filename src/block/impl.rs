use crate::block::{Block, Due};

/// Block-specific methods
pub trait BlockImpl {
    /// Produce a raw string representation of the block.
    #[must_use]
    fn raw(&self) -> String;
    /// Produce a plaintext representation of the block.
    #[must_use]
    fn plain(&self) -> String {
        if let Ok((text, _)) = Due::extract_and(&self.raw()) {
            text
        } else {
            self.raw()
        }
    }
    /// Return this block's [`Due`] attribute, if it exists.
    fn due(&self) -> Option<Due> {
        Due::extract_and(&self.raw()).ok().map(|(_, due)| due)
    }
}

impl BlockImpl for Block<'_> {
    fn raw(&self) -> String {
        match self {
            Self::Text(text, _) => text.raw(),
            Self::Task(task, _) => task.raw(),
        }
    }
    fn plain(&self) -> String {
        match self {
            Self::Text(text, _) => text.plain(),
            Self::Task(task, _) => task.plain(),
        }
    }
    fn due(&self) -> Option<Due> {
        match self {
            Self::Text(text, _) => text.due(),
            Self::Task(task, _) => task.due(),
        }
    }
}
