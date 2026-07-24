#![warn(clippy::unwrap_used)]
#![doc = include_str!("../README.md")]

/// In [Logseq](https://logseq.com) (and other outliners), each bullet is a "block." For our purposes, a block ([`block::Block`]) is either [`block::Text`] or [`block::Task`]
pub mod block;
pub(crate) mod consts;
/// Error types
pub mod error;
/// Graph-related code, and the main entrypoint of this library
pub mod graph;

/// Some functions need the [`comrak::Arena`] type. This module exposes the entire [`comrak`] library so you don't need to add it to your own crate to use it
pub mod ext {
    pub use comrak;
}

/// Convenience module for glob imports (`use alleged_lib::prelude::*`)
pub mod prelude {
    pub use crate::block::*;
    pub use crate::error::*;
    pub use crate::ext::comrak::Arena;
    pub use crate::graph::*;
}
