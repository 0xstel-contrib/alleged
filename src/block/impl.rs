use crate::block::Due;

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
