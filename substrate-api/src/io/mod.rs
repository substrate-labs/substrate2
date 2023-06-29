//! Traits and types for defining interfaces and signals in Substrate.

use std::{
    borrow::Borrow,
    collections::HashMap,
    ops::{Deref, Index},
};

use arcstr::ArcStr;
use geometry::transform::{HasTransformedView, Transformed};
use serde::{Deserialize, Serialize};
use tracing::Level;

use crate::{
    error::Result,
    layout::{element::Shape, error::LayoutError},
};

mod impls;

// BEGIN TRAITS

/// A trait implemented by block input/output interfaces.
pub trait Io: Directed + SchematicType + LayoutType {
    // TODO
}

/// Indicates that a hardware type specifies signal directions for all of its fields.
pub trait Directed: Flatten<Direction> {}
impl<T: Flatten<Direction>> Directed for T {}

/// A marker trait indicating that a hardware type does not specify signal directions.
pub trait Undirected {}

/// Flatten a structure into a list.
pub trait Flatten<T>: FlatLen {
    /// Flatten a structure into a list.
    fn flatten<E>(&self, output: &mut E)
    where
        E: Extend<T>;

    /// Flatten into a [`Vec`].
    fn flatten_vec(&self) -> Vec<T> {
        let len = self.len();
        let mut vec = Vec::with_capacity(len);
        self.flatten(&mut vec);
        assert_eq!(vec.len(), len, "Flatten::flatten_vec produced a Vec with an incorrect length: expected {} from FlatLen::len, got {}", len, vec.len());
        vec
    }
}

/// The length of the flattened list.
pub trait FlatLen {
    /// The length of the flattened list.
    fn len(&self) -> usize;
    /// Whether or not the flattened representation is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// An object with named flattened components.
pub trait HasNameTree {
    /// Return a tree specifying how nodes contained within this type should be named.
    ///
    /// Important: empty types (i.e. those with a flattened length of 0) must return [`None`].
    /// All non-empty types must return [`Some`].
    fn names(&self) -> Option<Vec<NameTree>>;

    /// Returns a flattened list of node names.
    fn flat_names(&self, root: impl Into<NameFragment>) -> Vec<NameBuf> {
        self.names()
            .map(|t| NameTree::new(root.into(), t).flatten())
            .unwrap_or_default()
    }
}

/// A schematic hardware type.
pub trait SchematicType: FlatLen + HasNameTree + Clone {
    /// The **Rust** type representing schematic instances of this **hardware** type.
    type Data: SchematicData;

    /// Instantiates a schematic data struct with populated nodes.
    ///
    /// Must consume exactly [`FlatLen::len`] elements of the node list.
    fn instantiate<'n>(&self, ids: &'n [Node]) -> (Self::Data, &'n [Node]);
}

/// A trait indicating that this type can be connected to T.
pub trait Connect<T> {}

/// A layout hardware type.
pub trait LayoutType: FlatLen + HasNameTree + Clone {
    /// The **Rust** type representing layout instances of this **hardware** type.
    type Data: LayoutData;
    /// The **Rust** type representing layout instances of this **hardware** type.
    type Builder: LayoutDataBuilder<Self::Data>;

    /// Instantiates a schematic data struct with populated nodes.
    fn builder(&self) -> Self::Builder;
}

/// Schematic hardware data.
///
/// An instance of a [`SchematicType`].
pub trait SchematicData: FlatLen + Flatten<Node> {}
impl<T> SchematicData for T where T: FlatLen + Flatten<Node> {}

/// Layout hardware data.
///
/// An instance of a [`LayoutType`].
pub trait LayoutData: FlatLen + Flatten<PortGeometry> + HasTransformedView + Send + Sync {}
impl<T> LayoutData for T where T: FlatLen + Flatten<PortGeometry> + HasTransformedView + Send + Sync {}

/// Layout hardware data builder.
///
/// A builder for an instance of a [`LayoutData`].
pub trait LayoutDataBuilder<T: LayoutData>: FlatLen {
    /// Builds an instance of [`LayoutData`].
    fn build(self) -> Result<T>;
}

// END TRAITS

// BEGIN TYPES

/// A portion of a node name.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum NameFragment {
    /// An element identified by a string name, such as a struct field.
    Str(ArcStr),
    /// A numbered element of an array/bus.
    Idx(usize),
}

/// An owned node name, consisting of an ordered list of [`NameFragment`]s.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct NameBuf {
    fragments: Vec<NameFragment>,
}

/// A tree for hierarchical node naming.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct NameTree {
    fragment: NameFragment,
    children: Vec<NameTree>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
/// An input port of hardware type `T`.
pub struct Input<T: Undirected>(pub T);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
/// An output port of hardware type `T`.
pub struct Output<T: Undirected>(pub T);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
/// An inout port of hardware type `T`.
pub struct InOut<T: Undirected>(pub T);

/// A transformed input port of hardware type `T`.
pub struct TransformedInput<'a, T: Undirected + HasTransformedView + 'a>(pub Transformed<'a, T>);

/// An transformed output port of hardware type `T`.
pub struct TransformedOutput<'a, T: Undirected + HasTransformedView + 'a>(pub Transformed<'a, T>);

/// An transformed inout port of hardware type `T`.
pub struct TransformedInOut<'a, T: Undirected + HasTransformedView + 'a>(pub Transformed<'a, T>);

/// A type representing a single hardware wire.
#[derive(Debug, Default, Clone, Copy)]
pub struct Signal;

/// A single node in a circuit.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Node(u32);

/// The priority a node has in determining the name of a merged node.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub(crate) enum NodePriority {
    /// An IO / externally-visible signal name.
    ///
    /// Has the highest priority in determining node names.
    Io = 3,
    /// An explicitly named signal.
    Named = 2,
    /// A signal with an automatically-generated name.
    ///
    /// Has the lowest priority in determining node names.
    Auto = 1,
}

/// The value associated to a node in a schematic builder's union find data structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[doc(hidden)]
pub struct NodeUfValue {
    /// The overall priority of a set of merged nodes.
    ///
    /// Taken to be the highest among priorities of all nodes
    /// in the merged set.
    priority: NodePriority,
    /// The node that provides `priority`.
    ///
    /// For example, if priority is NodePriority::Io, `node`
    /// should be the node identifier representing the IO node.
    pub(crate) source: Node,
}

/// A node unification table for connectivity management.
pub type NodeUf = ena::unify::InPlaceUnificationTable<Node>;

impl ena::unify::UnifyValue for NodeUfValue {
    type Error = ena::unify::NoError;

    fn unify_values(value1: &Self, value2: &Self) -> std::result::Result<Self, Self::Error> {
        Ok(if value1.priority >= value2.priority {
            *value1
        } else {
            *value2
        })
    }
}

impl ena::unify::UnifyKey for Node {
    type Value = Option<NodeUfValue>;
    fn index(&self) -> u32 {
        self.0
    }

    fn from_index(u: u32) -> Self {
        Self(u)
    }

    fn tag() -> &'static str {
        "Node"
    }
}

pub(crate) struct NodeContext {
    uf: NodeUf,
}

impl NodeContext {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            uf: Default::default(),
        }
    }
    pub(crate) fn node(&mut self, priority: NodePriority) -> Node {
        let id = self.uf.new_key(Default::default());
        self.uf.union_value(
            id,
            Some(NodeUfValue {
                priority,
                source: id,
            }),
        );
        id
    }
    #[inline]
    pub fn into_inner(self) -> NodeUf {
        self.uf
    }
    pub fn nodes(&mut self, n: usize, priority: NodePriority) -> Vec<Node> {
        (0..n).map(|_| self.node(priority)).collect()
    }
    pub(crate) fn connect(&mut self, n1: Node, n2: Node) {
        self.uf.union(n1, n2);
    }
}

/// A set of geometry associated with a layout port.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct PortGeometry {
    /// The primary shape of the port.
    ///
    /// This field is a copy of a shape contained in one of the other fields, so it is not drawn
    /// explicitly. It is kept separately for ease of access.
    primary: Shape,
    unnamed_shapes: Vec<Shape>,
    named_shapes: HashMap<ArcStr, Shape>,
}

/// A set of transformed geometry associated with a layout port.
#[allow(dead_code)]
pub struct TransformedPortGeometry<'a> {
    /// The primary shape of the port.
    ///
    /// This field is a copy of a shape contained in one of the other fields, so it is not drawn
    /// explicitly. It is kept separately for ease of access.
    pub primary: Shape,
    /// A set of unnamed shapes contained by the port.
    pub unnamed_shapes: Transformed<'a, [Shape]>,
    /// A set of named shapes contained by the port.
    pub named_shapes: Transformed<'a, HashMap<ArcStr, Shape>>,
}

/// A set of geometry associated with a layout port.
#[derive(Clone, Debug, Default)]
pub struct PortGeometryBuilder {
    primary: Option<Shape>,
    unnamed_shapes: Vec<Shape>,
    named_shapes: HashMap<ArcStr, Shape>,
}

impl PortGeometryBuilder {
    /// Push an unnamed shape to the port.
    ///
    /// If the primary shape has not been set yet, sets the primary shape to the new shape. This
    /// can be overriden using [`PortGeometryBuilder::set_primary`].
    pub fn push(&mut self, shape: Shape) {
        if self.primary.is_none() {
            self.primary = Some(shape.clone());
        }
        self.unnamed_shapes.push(shape);
    }

    /// Sets the primary shape of this port.
    pub fn set_primary(&mut self, shape: Shape) {
        self.primary = Some(shape);
    }
}

/// Port directions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub enum Direction {
    /// Input.
    Input,
    /// Output.
    Output,
    /// Input or output.
    ///
    /// Represents ports whose direction is not known
    /// at generator elaboration time.
    #[default]
    InOut,
}

impl Direction {
    /// Returns the flipped direction.
    ///
    /// [`Direction::InOut`] is unchanged by flipping.
    ///
    /// # Examples
    ///
    /// ```
    /// use substrate::io::Direction;
    /// assert_eq!(Direction::Input.flip(), Direction::Output);
    /// assert_eq!(Direction::Output.flip(), Direction::Input);
    /// assert_eq!(Direction::InOut.flip(), Direction::InOut);
    /// ```
    #[inline]
    pub fn flip(&self) -> Self {
        match *self {
            Self::Input => Self::Output,
            Self::Output => Self::Input,
            Self::InOut => Self::InOut,
        }
    }
}

/// A signal exposed by a cell.
#[allow(dead_code)]
pub struct Port {
    direction: Direction,
    node: Node,
}

/// An array containing some number of elements of type `T`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct Array<T> {
    len: usize,
    ty: T,
}

impl<T> Array<T> {
    /// Create a new array of the given length and hardware type.
    #[inline]
    pub fn new(len: usize, ty: T) -> Self {
        Self { len, ty }
    }
}

/// An instantiated array containing a fixed number of elements of type `T`.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct ArrayData<T> {
    elems: Vec<T>,
    ty_len: usize,
}

// END TYPES
