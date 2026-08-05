mod due;
mod task;

pub use due::*;
pub use task::*;

pub trait BlockLabel: Sized {
    type Error;

    /// Helper for `(implementor)::from_str` that returns a mutated (e.g. cleaned) version of the input.
    ///
    /// # Errors
    /// Errors if the implementor's [`std::str::FromStr`] call fails.
    fn extract_modify(s: &str) -> Result<(String, Self), Self::Error>;
}
