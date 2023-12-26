//! Atoll: Automatic transformation of logical layout.
//!
//! Atoll projects are made of one or more **blocks**.
//! Each block is a compact, rectangular grid of devices.
//! Each block in turn is composed of a set of tiles drawn from a TileSet.
//! TileSets provide a tile generator for each tile archetype.
//!
//! The set of tile archetypes is given by the Cartesian product
//! of [`Col`] and [`Row`].
//!
//! A tile generator takes tile configuration info and produces
//! a tile of its archetype.
//!
//! # Grid structure
//!
//! Inter-tile and inter-block routes are drawn on designated routing layers.
//! Intra-tile routing can be done on any layer; Atoll does not interface with such routing.
//!
//! Atoll assumes that you have a set of metal layers `M0, M1, M2, ...`, where each metal
//! layer can be connected to the layer immediately above or below.
//! Atoll also assumes that each metal layer has a preferred direction, and that
//! horizontal and vertical metals alternate.
//!
//! Blocks should leave some layers available for inter-block routing.
//!
//! Suppose that `P(i)` is the pitch of the i-th routing layer.
//! TileSets must pick an integer X such that:
//! * A complete block (including intra-block routing) can be assembled from layers `M0, M1, ..., MX`
//! * In particular, tiles contain no routing layers above `MX`.
//! * The width of all tiles is an integer multiple of `LCM { P(0), P(2), ... }`,
//!   assuming `M0` is vertical.
//! * The height of all tiles is an integer multiple of `LCM { P(1), P(3), ... }`,
//!   assuming `M0` is vertical (and therefore that `M1` is horizontal).
//!
//! All routing tracks must be fully internal to each tile.
//! The line and space widths must each be even integers, so that the center
//! of any track or space is also an integer.
//!
//! When the ratio `P(L+2) / P(L)` is not an integer, Atoll's routing algorithms assume
//! that if track `T` on layer `L+2` lies strictly between tracks `U1` and `U2` on layer `L`,
//! and track `T` makes a connection to track `V` on layer `L+1`, then the grid points
//! `(V, U1)` and `(V, U2)` must be left unused or must be connected to the same net as `(T, V)`.
//!
//! ## Track numbering
//!
//! Track coordinates have the form `(layer, x, y)`.
//! Each track coordinate references an intersection point between a track
//! on `layer` and a track on `layer + 1`.
//! If `layer` runs horizontally, `x` indexes the (vertical) tracks on `layer + 1`
//! and `y` indexes the horizontal tracks on `layer`.
//! The origin is the lower-left corner of the tile.
//!
//! # Tiles
//!
//! Each tile is conceptually composed of two slices: a device slice, and a routing slice.
//!
//! ## Device slices
//!
//! The device slice encompasses structures placed on base layers.
//! Typical device slices may produce:
//! * Transistors
//! * Resistors
//! * Capacitors
//! * Taps
//!
//! The device slice may perform some intra-device routing.
//! The device slice is responsible for connecting signals that must be exposed
//! to tracks in the routing slice.
//!
//! ## Routing slices
//!
//! Routing slices are responsible for:
//! * Bringing intra-device signals that need to be exposed to an edge track.
//! * Selecting routing paths for signals that go through a tile.
//!
//! A track is considered an edge track if at least one adjacent track on the
//! same layer falls outside the tile's boundary.
//!
//! # Routing
//!
//! There are three phases of routing:
//! * Global routing
//! * Intra-tile routing
//! * Inter-tile routing
//! * Inter-block routing
//!
//! Global routing assigns nets/devices to blocks and creates cut-through routes.
//!
//! Intra-tile routing brings all exposed device slice signals to
//! one or more edge tracks within the tile.
//!
//! Inter-tile routing connects signals within a block.
//! The inter-tile router accepts a list of signals that must be exposed for
//! inter-block routing, along with an optional preferred edge (top, bottom, left, or right)
//! for where those signals should be exposed.
//!
//! Each tile communicates a list of obstructed track coordinates to the inter-tile router.
//!
//! Inter-block routing connects signals across blocks.
//!
//! ## Cut-through routes
//!
//! It is sometimes necessary to route a signal through a tile on layers
//! that the tile itself may be using. To do this, the global router can
//! instruct the inter-tile router to create a cut-through route on a
//! specific layer.
//!
//! The inter-tile router is then responsible for routing a track on the given
//! layer from one side of the block to a track on the same layer exiting on the opposite
//! side of the block. Note that the entry and exit track indices need not be the same.
//!
//! For example, a cut-through route may enter on track 1 on the left side of a block
//! and exit on track 2 on the same layer on the right side of the block.
//!
//! Cut-through routes can be created for signals that are internally used by a block.
//! These routes enter on one side of a block, may branch to zero or more devices within the block,
//! and exit on the same layer on the other side of the block.
//!
//! ## Filler cells
//!
//! Filler cells (e.g. tap and decap cells) must have a width
//! equal to the GCD of the widths of all device cells.
//! Note that this GCD must be an integer multiple of the LCMs of
//! track pitches over all vertical running routing layers.
//! A similar requirement holds for filler cell height.
//!
//! # Power strapping
//!
//! Atoll can be configured to insert power straps on tracks
//! available after routing.
//!
//! Nonuniform power straps are only supported during inter-block routing,
//! and only for layers above `MX`.
//!
//! The inter-tile router supports 3 power strap modes:
//! * Straps first: gives priority to straps, adding obstructions to the routing grid
//!   where a signal track overlaps or is otherwise too close to a power strap.
//! * Grid adjust: makes the signal routing grid non-uniform so that signal tracks
//!   do not collide with power straps.
//! * Straps last: performs inter-tile routing first, then adds straps wherever
//!   possible, without disturbing routed signals.
//!
#![warn(missing_docs)]

pub mod grid;

use ::grid::Grid;
use derive_where::derive_where;
use serde::Serialize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use substrate::arcstr::ArcStr;
use substrate::block::Block;
use substrate::geometry::bbox::Bbox;
use substrate::geometry::polygon::Polygon;
use substrate::geometry::prelude::{Dir, Point, Transformation};
use substrate::geometry::rect::Rect;
use substrate::geometry::transform::{HasTransformedView, TransformMut, TranslateMut};
use substrate::io::layout::PortGeometry;
use substrate::io::{FlatLen, Flatten, Signal};
use substrate::layout::element::Shape;
use substrate::layout::tracks::{EnumeratedTracks, FiniteTracks, Tracks};
use substrate::layout::{Draw, DrawReceiver, ExportsLayoutData, Layout};
use substrate::pdk::layers::HasPin;
use substrate::pdk::Pdk;
use substrate::schematic::schema::Schema;
use substrate::schematic::{
    CellId, ExportsNestedData, HasNestedView, InstanceId, InstancePath, NodeGroup, Schematic,
};
use substrate::serde::Deserialize;
use substrate::{io, layout, schematic};

/// Identifies nets in a routing solver.
pub type NetId = usize;

/// Identifies a routing layer.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct LayerId(usize);

impl From<usize> for LayerId {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

/// A coordinate identifying a track position in a routing volume.
pub struct Coordinate {
    /// The lower metal layer.
    pub layer: LayerId,
    /// The x-coordinate.
    ///
    /// Indexes the vertical-traveling tracks.
    pub x: i64,
    /// The y-coordinate.
    ///
    /// Indexes the horizontal-traveling tracks.
    pub y: i64,
}

/// A type that contains an x-y coordinate.
pub trait Xy {
    /// Returns the coordinate represented by `self`.
    fn xy(&self) -> (i64, i64);
}

impl<T: Xy> Xy for &T {
    fn xy(&self) -> (i64, i64) {
        (*self).xy()
    }
}

impl Xy for Coordinate {
    fn xy(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl Xy for Point {
    fn xy(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl Xy for (i64, i64) {
    fn xy(&self) -> (i64, i64) {
        *self
    }
}

/// The state of a point on a routing grid.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PointState {
    /// The grid point is available for routing.
    Available,
    /// The grid point is obstructed.
    Obstructed,
    /// The grid point is occupied by a known net.
    Routed(NetId),
}

impl PointState {
    /// Whether or not the given point can be used to route the given net.
    pub fn is_available_for_net(&self, net: NetId) -> bool {
        match self {
            Self::Available => true,
            Self::Routed(n) => *n == net,
            Self::Obstructed => false,
        }
    }
}

/// Allowed track directions on a routing layer.
///
/// Adjacent routing layers must have alternating track directions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RoutingDir {
    /// Layer should be used for vertical routing.
    Vert,
    /// Layer should be used for horizontal routing.
    Horiz,
    /// Layer can be used for either horizontal or vertical routing.
    Any {
        /// The direction of the tracks that form the coordinate system for this layer.
        track_dir: Dir,
    },
}

impl RoutingDir {
    /// Whether or not this routing direction allows movement in the given direction.
    pub fn supports_dir(&self, dir: Dir) -> bool {
        match dir {
            Dir::Horiz => self.supports_horiz(),
            Dir::Vert => self.supports_vert(),
        }
    }
    /// Whether or not this routing direction allows horizontal movement.
    pub fn supports_horiz(&self) -> bool {
        matches!(*self, Self::Horiz | Self::Any { .. })
    }
    /// Whether or not this routing direction allows vertical movement.
    pub fn supports_vert(&self) -> bool {
        matches!(*self, Self::Vert | Self::Any { .. })
    }

    /// The direction in which tracks following this routing direction travel.
    pub fn track_dir(&self) -> Dir {
        match *self {
            Self::Vert => Dir::Vert,
            Self::Horiz => Dir::Horiz,
            Self::Any { track_dir } => track_dir,
        }
    }
}

/// A position within a routing volume.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    /// The routing layer.
    layer: LayerId,
    /// The x-coordinate.
    x: i64,
    /// The y-coordinate.
    y: i64,
}

impl Pos {
    /// Create a new [`Pos`].
    pub fn new(layer: impl Into<LayerId>, x: i64, y: i64) -> Self {
        Self {
            layer: layer.into(),
            x,
            y,
        }
    }

    /// The index of the track going in the specified direction.
    pub fn track_coord(&self, dir: Dir) -> i64 {
        match dir {
            Dir::Vert => self.x,
            Dir::Horiz => self.y,
        }
    }

    /// The index of the coordinate in the given direction.
    ///
    /// [`Dir::Horiz`] gives the x-coordinate;
    /// [`Dir::Vert`] gives the y-coordinate;
    pub fn coord(&self, dir: Dir) -> i64 {
        match dir {
            Dir::Horiz => self.x,
            Dir::Vert => self.y,
        }
    }

    /// Returns a new `Pos` with the given coordinate indexing tracks going in the given direction.
    pub fn with_track_coord(&self, dir: Dir, coord: i64) -> Self {
        let Pos { layer, x, y } = *self;
        match dir {
            Dir::Vert => Self { layer, x: coord, y },
            Dir::Horiz => Self { layer, x, y: coord },
        }
    }

    /// Returns a new `Pos` with the given coordinate in the given direction.
    ///
    /// If `dir` is [`Dir::Horiz`], `coord` is taken as the new x coordinate.
    /// If `dir` is [`Dir::Vert`], `coord` is taken as the new y coordinate.
    pub fn with_coord(&self, dir: Dir, coord: i64) -> Self {
        let Pos { layer, x, y } = *self;
        match dir {
            Dir::Horiz => Self { layer, x: coord, y },
            Dir::Vert => Self { layer, x, y: coord },
        }
    }
}

// todo: how to connect by abutment (eg body terminals)

/// The abstract view of an ATOLL tile.
pub struct AtollAbstract {
    /// The topmost ATOLL layer used within the tile.
    top_layer: usize,
    /// The lower left corner of the tile, in LCM units with respect to `top_layer`.
    ll: Point,
    /// The upper right corner of the tile, in LCM units with respect to `top_layer`.
    ur: Point,
    /// The state of each layer, up to and including `top_layer`.
    layers: Vec<LayerAbstract>,
}

/// The abstracted state of a single routing layer.
pub enum LayerAbstract {
    /// The layer is fully blocked.
    ///
    /// No routing on this layer is permitted.
    Blocked,
}

pub struct AtollBuilder<'a, T: ExportsNestedData, S: Schema, PDK: Pdk> {
    builder: &'a mut layout::CellBuilder<PDK>,
    raw: schematic::RawCell<S>,
    cell: schematic::Cell<T>,
    connections: HashMap<NodeGroup, Vec<PortGeometry>>,
}

pub struct Instance<T: ExportsLayoutData>(layout::Instance<T>, Vec<io::schematic::NestedNode>);

impl<T: ExportsLayoutData> Clone for Instance<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T: ExportsLayoutData> Bbox for Instance<T> {
    fn bbox(&self) -> Option<Rect> {
        self.0.bbox()
    }
}

impl<T: ExportsLayoutData> TranslateMut for Instance<T> {
    fn translate_mut(&mut self, p: Point) {
        self.0.translate_mut(p)
    }
}

impl<T: ExportsLayoutData> TransformMut for Instance<T> {
    fn transform_mut(&mut self, trans: Transformation) {
        self.0.transform_mut(trans)
    }
}

impl<T: ExportsLayoutData> Instance<T> {
    pub fn into_inner(self) -> layout::Instance<T> {
        self.0
    }
}

impl<PDK: Pdk, T: Layout<PDK>> Draw<PDK> for Instance<T> {
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> substrate::error::Result<()> {
        self.0.draw(recv)
    }
}

impl<'a, T: Schematic<S>, S: Schema, PDK: Pdk> AtollBuilder<'a, T, S, PDK> {
    pub fn new(
        block: T,
        builder: &'a mut layout::CellBuilder<PDK>,
    ) -> substrate::error::Result<Self> {
        let cell = builder.ctx.generate_schematic(block);
        Ok(Self {
            builder,
            raw: cell.try_raw_cell()?.clone(),
            cell: cell.try_cell()?.clone(),
            connections: HashMap::new(),
        })
    }

    pub fn linked_generate<I: ExportsNestedData + Clone + Layout<PDK>>(
        &mut self,
        instance: schematic::NestedInstance<I>,
    ) -> Instance<I> {
        Instance(
            self.builder.generate(instance.block().clone()),
            instance.io_test().flatten_vec(),
        )
    }

    pub fn draw<I: Layout<PDK>>(&mut self, instance: Instance<I>) -> substrate::error::Result<()> {
        let ports: Vec<PortGeometry> = instance.0.try_io()?.flatten_vec();
        for (port, node) in ports.into_iter().zip(instance.1.into_iter()) {
            let group = self.raw.node_group(&node);
            self.connections
                .entry(group)
                .or_insert(Vec::new())
                .push(port);
        }

        self.builder.draw(instance.0)?;
        Ok(())
    }

    pub fn route(self) -> substrate::error::Result<()> {
        for (_, ports) in self.connections {
            for pair in ports.windows(2) {
                let a = &pair[0];
                let b = &pair[1];
                let a_center = a.primary.shape().bbox().unwrap().center();
                let b_center = b.primary.shape().bbox().unwrap().center();
                self.builder.draw(Shape::new(
                    a.primary.layer().pin(),
                    Polygon::from_verts(vec![
                        a_center,
                        b_center,
                        b_center - Point::new(0, 20),
                        a_center - Point::new(0, 20),
                    ]),
                ))?;
            }
        }
        Ok(())
    }

    pub fn cell(&self) -> &schematic::Cell<T> {
        &self.cell
    }
}
