use gray_matter;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PageError {
    #[error("Failed to parse page properties! ({0})")]
    Matter(#[from] gray_matter::Error),
}
