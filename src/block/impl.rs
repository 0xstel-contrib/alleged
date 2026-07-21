// Do we even need these methods?
/// Block-specific methods
pub trait BlockImpl {
    #[must_use]
    /// Produce a raw string representation of the block.
    fn raw(&self) -> String;
    #[must_use]
    /// Produce a plaintext representation of the block.
    fn plain(&self) -> String;
}
