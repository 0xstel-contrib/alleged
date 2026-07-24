mod block;
pub use block::*;

pub trait BlockPropertyImpl: Sized {
    type Error;

    fn extract_and(s: &str) -> Result<(String, Self), Self::Error>;
}
