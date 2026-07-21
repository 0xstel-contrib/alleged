// Do we even need these methods?
pub trait BlockImpl {
    #[must_use]
    fn raw(&self) -> String;
    #[must_use]
    fn plain(&self) -> String;
}
