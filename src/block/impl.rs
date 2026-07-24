use crate::block::{Block, BlockProperties, BlockPropertyImpl, Due};
#[cfg(feature = "hash")]
use twox_hash::XxHash3_128;

/// Block-specific methods
pub trait BlockImpl {
    /// Produce a raw string representation of the block.
    #[must_use]
    fn raw(&self) -> String;
    /// Produce a plaintext representation of the block.
    #[must_use]
    fn plain(&self) -> String {
        let content_text = self.excluding_properties();
        if let Ok((text, _)) = Due::extract_and(&content_text) {
            text
        } else {
            content_text
        }
    }
    #[must_use]
    fn excluding_properties(&self) -> String {
        BlockProperties::extract_and(&self.raw())
            .map(|(text, _)| text)
            .unwrap()
    }
    /// Return this block's [`Due`] attribute, if it exists.
    fn due(&self) -> Option<Due> {
        Due::extract_and(&self.excluding_properties())
            .ok()
            .map(|(_, due)| due)
    }
    /// Return a deterministic ID for this block.
    #[cfg(feature = "hash")]
    fn hash(&self) -> String {
        hex::encode(XxHash3_128::oneshot(self.raw().as_bytes()).to_le_bytes())
    }
    fn properties(&self) -> BlockProperties {
        let raw = self.raw();
        BlockProperties::extract_and(&raw)
            .map(|(_, properties)| properties)
            .unwrap()
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
