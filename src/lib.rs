#![warn(clippy::unwrap_used)]

pub mod block;
pub mod consts;
pub mod error;
pub mod graph;
pub mod properties;

pub mod prelude {
    pub use crate::block::*;
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::graph::*;
    pub use crate::properties::*;
}

pub mod ext {
    pub use comrak;
}
