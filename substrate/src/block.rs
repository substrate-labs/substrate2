//! A block that can be instantiated by Substrate.

use std::{any::Any, hash::Hash};

use arcstr::ArcStr;
pub use codegen::Block;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::io::Io;
use crate::sealed;
use crate::sealed::Token;

/// The kind of a Substrate [`Block`].
///
/// May be one of [`Cell`], [`Scir`], and [`Primitive`].
pub trait BlockKind {
    #[doc(hidden)]
    fn sealed(_: sealed::Token);
}

/// A cell formed by composing Substrate blocks.
pub struct Cell;
impl BlockKind for Cell {
    fn sealed(_: Token) {}
}

/// A block associated with a SCIR cell.
///
/// Used for hard macros that can be parsed
/// to SCIR libraries.
pub struct Scir;
impl BlockKind for Scir {
    fn sealed(_: Token) {}
}

/// A block associated with a schema primitive.
///
/// Used for blocks that cannot be expressed as
/// SCIR cells or compositions of other Substrate
/// blocks.
pub struct Primitive;
impl BlockKind for Primitive {
    fn sealed(_: Token) {}
}

/// A block that can be instantiated by Substrate.
///
/// # Examples
///
#[doc = examples::get_snippets!("core", "inverter")]
pub trait Block: Serialize + DeserializeOwned + Hash + Eq + Send + Sync + Any {
    /// The kind of this block.
    type Kind: BlockKind;
    /// The ports of this block.
    type Io: Io;

    /// A crate-wide unique identifier for this block.
    fn id() -> ArcStr;

    /// A name for a specific parametrization of this block.
    ///
    /// Instances of this block will initially be assigned this name,
    /// although Substrate may need to change the name
    /// (e.g. to avoid duplicates).
    fn name(&self) -> ArcStr {
        arcstr::literal!("unnamed")
    }

    /// Returns a fully-specified instance of this cell's `Io`.
    fn io(&self) -> Self::Io;
}
