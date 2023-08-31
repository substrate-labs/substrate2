//! Substrate's schematic generator framework.

pub mod conv;
pub mod primitives;
pub mod schema;

use cache::error::TryInnerError;
use cache::mem::TypeCache;
use cache::CacheHandle;
pub use codegen::{Schematic, SchematicData};
use pathtree::PathTree;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;

use arcstr::ArcStr;
use schema::Schema;
use scir::Library;
use substrate::pdk::PdkScirSchematic;
use type_dispatch::impl_dispatch;

use crate::block::{self, Block, Opaque, PdkPrimitive, ScirBlock};
use crate::context::Context;
use crate::diagnostics::SourceInfo;
use crate::error::{Error, Result};
use crate::io::{
    Connect, HasNameTree, HasTerminalView, NameBuf, Node, NodeContext, NodePriority, NodeUf, Port,
    SchematicBundle, SchematicType, TerminalView,
};
use crate::pdk::{Pdk, PdkSchematic, ToSchema};

pub trait Primitive: Clone + Send + Sync + Any {}

impl<T: Clone + Send + Sync + Any> Primitive for T {}

/// A block with a schematic specified using SCIR.
pub trait ScirSchematic<PDK: Pdk, S: Schema, K = <Self as Block>::Kind>: ScirBlock {
    /// Returns the library containing the SCIR cell and its ID.
    fn schematic(&self) -> Result<(Library<S::Primitive>, scir::CellId)>;
}

impl<PDK: ToSchema<S>, S: Schema, B: Block<Kind = block::PdkScir> + PdkScirSchematic<PDK>>
    ScirSchematic<PDK, S, block::PdkScir> for B
{
    fn schematic(&self) -> Result<(Library<S::Primitive>, scir::CellId)> {
        let (lib, cell) = PdkScirSchematic::schematic(self)?;
        Ok((
            lib.convert_primitives(PDK::convert_primitive)
                .ok_or(Error::UnsupportedPrimitive)?,
            cell,
        ))
    }
}

/// A block whose contents are opaque to Substrate.
pub trait Blackbox<PDK: Pdk, S: Schema>: Block<Kind = Opaque> {
    /// Returns the contents of the blackbox.
    fn contents(&self, io: &<<Self as Block>::Io as SchematicType>::Bundle) -> BlackboxContents;
}

/// A block that exports nodes from its schematic.
///
/// All blocks that have a schematic implementation must export nodes.
pub trait ExportsNestedNodes<K = <Self as Block>::Kind>: Block {
    /// Extra schematic data to be stored with the block's generated cell.
    ///
    /// When the block is instantiated, all contained data will be nested
    /// within that instance.
    type NestedNodes: NestedNodes;
}

/// A block that has a schematic associated with the given PDK and schema.
pub trait Schematic<PDK: Pdk, S: Schema, K = <Self as Block>::Kind>: ExportsNestedNodes {
    /// Generates the block's schematic.
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<PDK, S>,
    ) -> Result<Self::NestedNodes>;
}

impl<B: Block<Kind = block::Scir>> ExportsNestedNodes<block::Scir> for B {
    type NestedNodes = ();
}

#[impl_dispatch({block::Scir; block::PdkScir})]
impl<
        T,
        PDK: Pdk,
        S: Schema,
        B: Block<Kind = T> + ExportsNestedNodes<NestedNodes = ()> + ScirSchematic<PDK, S>,
    > Schematic<PDK, S, T> for B
{
    fn schematic(
        &self,
        _io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<PDK, S>,
    ) -> Result<Self::NestedNodes> {
        let (lib, id) = ScirSchematic::schematic(self)?;
        cell.0.set_scir(ScirCellInner {
            lib: Arc::new(lib),
            cell: id,
        });
        Ok(())
    }
}

#[impl_dispatch({PdkPrimitive; block::PdkCell})]
impl<T, PDK: ToSchema<S>, S: Schema, B: PdkSchematic<PDK, T> + Block<Kind = T>> Schematic<PDK, S, T>
    for B
{
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<PDK, S>,
    ) -> Result<Self::NestedNodes> {
        cell.set_pdk_schematic(io, self)
    }
}
impl<B: Block<Kind = Opaque>> ExportsNestedNodes<Opaque> for B {
    type NestedNodes = ();
}
impl<PDK: Pdk, S: Schema, B: Blackbox<PDK, S>> Schematic<PDK, S, Opaque> for B {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<PDK, S>,
    ) -> Result<Self::NestedNodes> {
        cell.0.set_blackbox(Blackbox::contents(self, io));
        Ok(())
    }
}

/// A builder for creating a schematic cell.
pub(crate) struct CellBuilderInner<PDK: Pdk, P> {
    /// The current global context.
    pub ctx: Context<PDK>,
    pub(crate) id: CellId,
    pub(crate) cell_name: ArcStr,
    pub(crate) flatten: bool,
    /// The root instance path that all nested paths should be relative to.
    pub(crate) root: InstancePath,
    pub(crate) node_ctx: NodeContext,
    pub(crate) node_names: HashMap<Node, NameBuf>,
    /// Outward-facing ports of this cell.
    ///
    /// Directions are as viewed by a parent cell instantiating this cell; these
    /// are the wrong directions to use when looking at connections to this
    /// cell's IO from *within* the cell.
    pub(crate) ports: Vec<Port>,
    pub(crate) contents: CellBuilderContents<P>,
}

impl<PDK: Pdk, P> CellBuilderInner<PDK, P> {
    fn clone_without_contents(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            id: self.id,
            cell_name: self.cell_name.clone(),
            flatten: self.flatten,
            /// The root instance path that all nested paths should be relative to.
            root: self.root.clone(),
            node_ctx: self.node_ctx.clone(),
            node_names: self.node_names.clone(),
            /// Outward-facing ports of this cell.
            ///
            /// Directions are as viewed by a parent cell instantiating this cell; these
            /// are the wrong directions to use when looking at connections to this
            /// cell's IO from *within* the cell.
            ports: self.ports.clone(),
            contents: CellBuilderContents::Cell(CellInner {
                next_instance_id: InstanceId(0),
                instances: Vec::new(),
            }),
        }
    }
}

pub struct CellBuilder<PDK: Pdk, S: Schema>(
    pub(crate) CellBuilderInner<PDK, S::Primitive>,
    PhantomData<S>,
);
pub struct PdkCellBuilder<PDK: Pdk>(pub(crate) CellBuilderInner<PDK, PDK::Primitive>);

pub(crate) type CellBuilderContents<P> =
    RawCellKind<CellInner<P>, ScirCellInner<P>, P, BlackboxContents>;

impl<P: Primitive> CellBuilderContents<P> {
    fn convert_primitives<C>(
        self,
        convert_fn: fn(P) -> Option<C>,
    ) -> Option<CellBuilderContents<C>> {
        Some(match self {
            Self::Cell(c) => CellBuilderContents::Cell(c.convert_primitives(convert_fn)?),
            Self::Scir(c) => CellBuilderContents::Scir(c.convert_primitives(convert_fn)?),
            Self::Primitive(c) => CellBuilderContents::Primitive(convert_fn(c)?),
            Self::Blackbox(c) => CellBuilderContents::Blackbox(c),
        })
    }
}

/// An enumeration of raw cell kinds.
///
/// Can be used to store data associated with each kind of raw cell.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[enumify::enumify]
pub(crate) enum CellBuilderKind<C, S, P, B> {
    PdkCell(C),
    Cell(C),
    Scir(S),
    Primitive(P),
    Blackbox(B),
}

pub(crate) struct CellInner<P> {
    pub(crate) next_instance_id: InstanceId,
    pub(crate) instances: Vec<Receiver<Option<RawInstance<P>>>>,
}

impl<P: Primitive> CellInner<P> {
    fn convert_primitives<C>(self, convert_fn: fn(P) -> Option<C>) -> Option<CellInner<C>> {
        Some(CellInner {
            next_instance_id: self.next_instance_id,
            instances: self
                .instances
                .into_iter()
                .map(|instance| {
                    instance
                        .recv()
                        .unwrap()
                        .unwrap()
                        .convert_primitives(convert_fn)
                })
                .collect::<Option<Vec<_>>>()?
                .into_iter()
                .map(|instance| {
                    let (send, recv) = mpsc::channel();
                    send.send(Some(instance)).unwrap();
                    recv
                })
                .collect(),
        })
    }
}

/// The contents of a blackbox cell.
#[derive(Debug, Default, Clone)]
pub struct BlackboxContents {
    /// The list of [`BlackboxElement`]s comprising this cell.
    ///
    /// During netlisting, each blackbox element will be
    /// injected into the final netlist.
    /// Netlister implementations should add spaces before each element
    /// in the list, except for the first element.
    pub elems: Vec<BlackboxElement>,
}

/// An element in the contents of a blackbox cell.
#[derive(Debug, Clone)]
pub enum BlackboxElement {
    /// A reference to a [`Node`].
    Node(Node),
    /// A raw, opaque [`String`].
    RawString(String),
}

impl BlackboxContents {
    /// Creates a new, empty [`BlackboxContents`].
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds the given element to the list of blackbox elements.
    pub fn push(&mut self, elem: impl Into<BlackboxElement>) {
        self.elems.push(elem.into());
    }
}

impl FromIterator<BlackboxElement> for BlackboxContents {
    fn from_iter<T: IntoIterator<Item = BlackboxElement>>(iter: T) -> Self {
        Self {
            elems: iter.into_iter().collect(),
        }
    }
}

impl From<String> for BlackboxElement {
    #[inline]
    fn from(value: String) -> Self {
        Self::RawString(value)
    }
}

impl From<&str> for BlackboxElement {
    #[inline]
    fn from(value: &str) -> Self {
        Self::RawString(value.to_string())
    }
}

impl From<Node> for BlackboxElement {
    #[inline]
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

impl From<&Node> for BlackboxElement {
    #[inline]
    fn from(value: &Node) -> Self {
        Self::Node(*value)
    }
}

impl From<String> for BlackboxContents {
    fn from(value: String) -> Self {
        Self {
            elems: vec![BlackboxElement::RawString(value)],
        }
    }
}

impl From<&str> for BlackboxContents {
    fn from(value: &str) -> Self {
        Self {
            elems: vec![BlackboxElement::RawString(value.to_string())],
        }
    }
}

impl<PDK: Pdk, P: Primitive> CellBuilderInner<PDK, P> {
    pub(crate) fn finish(self) -> RawCell<P> {
        let mut roots = HashMap::with_capacity(self.node_names.len());
        let mut uf = self.node_ctx.into_uf();
        for &node in self.node_names.keys() {
            let root = uf.probe_value(node).unwrap().source;
            roots.insert(node, root);
        }

        RawCell {
            id: self.id,
            name: self.cell_name,
            node_names: self.node_names,
            ports: self.ports,
            uf,
            roots,
            contents: self.contents.into(),
            flatten: self.flatten,
        }
    }

    /// Create a new signal with the given name and hardware type.
    #[track_caller]
    pub fn signal<TY: SchematicType>(
        &mut self,
        name: impl Into<ArcStr>,
        ty: TY,
    ) -> <TY as SchematicType>::Bundle {
        let (nodes, data) = self.node_ctx.instantiate_undirected(
            &ty,
            NodePriority::Named,
            SourceInfo::from_caller(),
        );

        let names = ty.flat_names(Some(name.into().into()));
        assert_eq!(nodes.len(), names.len());

        self.node_names.extend(nodes.iter().copied().zip(names));

        data
    }

    /// Connect all signals in the given data instances.
    pub fn connect<D1, D2>(&mut self, s1: D1, s2: D2)
    where
        D1: SchematicBundle,
        D2: SchematicBundle,
        D1: Connect<D2>,
    {
        let s1f: Vec<Node> = s1.flatten_vec();
        let s2f: Vec<Node> = s2.flatten_vec();
        assert_eq!(s1f.len(), s2f.len());
        s1f.into_iter().zip(s2f).for_each(|(a, b)| {
            // FIXME: proper error handling mechanism (collect all errors into
            // context and emit later)
            let res = self.node_ctx.connect(a, b);
            if let Err(err) = res {
                tracing::warn!(?err, "connection failed");
            }
        });
    }

    /// Marks this cell as a SCIR cell.
    pub(crate) fn set_scir(&mut self, scir: ScirCellInner<P>) {
        self.contents = CellBuilderContents::Scir(scir);
    }

    /// Marks this cell as a primitive.
    pub(crate) fn set_primitive(&mut self, primitive: P) {
        self.contents = CellBuilderContents::Primitive(primitive);
    }

    /// Marks this cell as a blackbox containing the given content.
    pub(crate) fn set_blackbox(&mut self, contents: impl Into<BlackboxContents>) {
        self.contents = CellBuilderContents::Blackbox(contents.into());
    }

    /// Gets the global context.
    pub(crate) fn ctx(&self) -> &Context<PDK> {
        &self.ctx
    }

    /// Starts generating a block in a new thread and returns a handle to its cell.
    ///
    /// Can be used to check data stored in the cell or other generated results before adding the
    /// cell to the current schematic with [`CellBuilder::add`].
    ///
    /// To generate and add the block simultaneously, use [`CellBuilder::instantiate`]. However,
    /// error recovery and other checks are not possible when using
    /// [`instantiate`](CellBuilder::instantiate).
    fn generate<S: Schema<Primitive = P>, I: Schematic<PDK, S>>(
        &mut self,
        block: I,
    ) -> CellHandle<S::Primitive, I> {
        self.ctx.generate_schematic(block)
    }

    /// Generates a cell corresponding to `block` and returns a handle to it.
    ///
    /// Blocks on generation. Useful for handling errors thrown by the generation of a cell immediately.
    ///
    /// As with [`CellBuilder::generate`], the resulting handle must be added to the schematic with
    /// [`CellBuilder::add`] before it can be connected as an instance.
    fn generate_blocking<S: Schema<Primitive = P>, I: Schematic<PDK, S>>(
        &mut self,
        block: I,
    ) -> Result<CellHandle<S::Primitive, I>> {
        let cell = self.ctx.generate_schematic(block);
        cell.try_cell()?;
        Ok(cell)
    }

    /// Adds a cell generated with [`CellBuilder::generate`] to the current schematic.
    ///
    /// Does not block on generation. Spawns a thread that waits on the generation of
    /// the underlying cell and panics if generation fails. If error recovery is desired,
    /// check errors before calling this function using [`CellHandle::try_cell`].
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    fn add<S: Schema<Primitive = P>, I: Schematic<PDK, S>>(
        &mut self,
        cell: CellHandle<S::Primitive, I>,
    ) -> Instance<S::Primitive, I> {
        self.post_instantiate(cell, SourceInfo::from_caller())
    }

    /// Instantiate a schematic view of the given block.
    ///
    /// This function generates and adds the cell to the schematic. If checks need to be done on
    /// the generated cell before it is added to the schematic, use [`CellBuilder::generate`] and
    /// [`CellBuilder::add`].
    ///
    /// Spawns a thread that generates the underlying cell and panics if the generator fails. If error
    /// recovery is desired, use the generate and add workflow mentioned above.
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    fn instantiate<S: Schema<Primitive = P>, I: Schematic<PDK, S>>(
        &mut self,
        block: I,
    ) -> Instance<S::Primitive, I> {
        let cell = self.ctx.generate_schematic(block);
        self.post_instantiate(cell, SourceInfo::from_caller())
    }

    /// Create an instance and immediately connect its ports.
    fn instantiate_connected<S: Schema<Primitive = P>, I, C>(&mut self, block: I, io: C)
    where
        I: Schematic<PDK, S>,
        C: SchematicBundle,
        <I::Io as SchematicType>::Bundle: Connect<C>,
    {
        let inst = self.instantiate(block);
        self.connect(inst.io, io);
    }

    /// Creates nodes for the newly-instantiated block's IOs.
    fn post_instantiate<I: ExportsNestedNodes>(
        &mut self,
        cell: CellHandle<P, I>,
        source_info: SourceInfo,
    ) -> Instance<P, I> {
        let io = cell.block.io();
        let cell_contents = self.contents.as_mut().unwrap_cell();

        let (nodes, io_data) =
            self.node_ctx
                .instantiate_directed(&io, NodePriority::Auto, source_info);

        let names = io.flat_names(Some(
            arcstr::format!("xinst{}", cell_contents.instances.len()).into(),
        ));
        assert_eq!(nodes.len(), names.len());

        self.node_names.extend(nodes.iter().copied().zip(names));

        cell_contents.next_instance_id.increment();

        let inst = Instance {
            id: cell_contents.next_instance_id,
            parent: self.root.clone(),
            path: self
                .root
                .append_segment(cell_contents.next_instance_id, cell.id),
            cell: cell.clone(),
            io: io_data,
        };

        let (send, recv) = mpsc::channel();

        cell_contents.instances.push(recv);

        thread::spawn(move || {
            if let Ok(cell) = cell.try_cell() {
                let raw = RawInstance {
                    id: inst.id,
                    name: arcstr::literal!("unnamed"),
                    child: cell.raw.clone(),
                    connections: nodes,
                };
                send.send(Some(raw)).unwrap();
            } else {
                send.send(None).unwrap();
            }
        });

        inst
    }
}

impl<PDK: Pdk<Primitive = P>, P: Primitive> CellBuilderInner<PDK, P> {
    /// Starts generating a block in a new thread and returns a handle to its cell.
    ///
    /// Can be used to check data stored in the cell or other generated results before adding the
    /// cell to the current schematic with [`CellBuilder::add`].
    ///
    /// To generate and add the block simultaneously, use [`CellBuilder::instantiate`]. However,
    /// error recovery and other checks are not possible when using
    /// [`instantiate`](CellBuilder::instantiate).
    fn generate_pdk<I: PdkSchematic<PDK>>(&mut self, block: I) -> CellHandle<PDK::Primitive, I> {
        self.ctx.generate_pdk_schematic(block)
    }

    /// Generates a cell corresponding to `block` and returns a handle to it.
    ///
    /// Blocks on generation. Useful for handling errors thrown by the generation of a cell immediately.
    ///
    /// As with [`CellBuilder::generate`], the resulting handle must be added to the schematic with
    /// [`CellBuilder::add`] before it can be connected as an instance.
    fn generate_pdk_blocking<I: PdkSchematic<PDK>>(
        &mut self,
        block: I,
    ) -> Result<CellHandle<PDK::Primitive, I>> {
        let cell = self.ctx.generate_pdk_schematic(block);
        cell.try_cell()?;
        Ok(cell)
    }

    /// Adds a cell generated with [`CellBuilder::generate`] to the current schematic.
    ///
    /// Does not block on generation. Spawns a thread that waits on the generation of
    /// the underlying cell and panics if generation fails. If error recovery is desired,
    /// check errors before calling this function using [`CellHandle::try_cell`].
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    fn add_pdk<I: PdkSchematic<PDK>>(
        &mut self,
        cell: CellHandle<PDK::Primitive, I>,
    ) -> Instance<PDK::Primitive, I> {
        self.post_instantiate(cell, SourceInfo::from_caller())
    }

    /// Instantiate a schematic view of the given block.
    ///
    /// This function generates and adds the cell to the schematic. If checks need to be done on
    /// the generated cell before it is added to the schematic, use [`CellBuilder::generate`] and
    /// [`CellBuilder::add`].
    ///
    /// Spawns a thread that generates the underlying cell and panics if the generator fails. If error
    /// recovery is desired, use the generate and add workflow mentioned above.
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    fn instantiate_pdk<I: PdkSchematic<PDK>>(&mut self, block: I) -> Instance<PDK::Primitive, I> {
        let cell = self.ctx.generate_pdk_schematic(block);
        self.post_instantiate(cell, SourceInfo::from_caller())
    }

    /// Create an instance and immediately connect its ports.
    fn instantiate_pdk_connected<I, C>(&mut self, block: I, io: C)
    where
        I: PdkSchematic<PDK>,
        C: SchematicBundle,
        <I::Io as SchematicType>::Bundle: Connect<C>,
    {
        let inst = self.instantiate_pdk(block);
        self.connect(inst.io, io);
    }
}

impl<PDK: Pdk> PdkCellBuilder<PDK> {
    /// Create a new signal with the given name and hardware type.
    #[track_caller]
    pub fn signal<TY: SchematicType>(
        &mut self,
        name: impl Into<ArcStr>,
        ty: TY,
    ) -> <TY as SchematicType>::Bundle {
        self.0.signal(name, ty)
    }

    /// Connect all signals in the given data instances.
    pub fn connect<D1, D2>(&mut self, s1: D1, s2: D2)
    where
        D1: SchematicBundle,
        D2: SchematicBundle,
        D1: Connect<D2>,
    {
        self.0.connect(s1, s2)
    }

    /// Gets the global context.
    pub fn ctx(&self) -> &Context<PDK> {
        self.0.ctx()
    }

    /// Starts generating a block in a new thread and returns a handle to its cell.
    ///
    /// Can be used to check data stored in the cell or other generated results before adding the
    /// cell to the current schematic with [`CellBuilder::add`].
    ///
    /// To generate and add the block simultaneously, use [`CellBuilder::instantiate`]. However,
    /// error recovery and other checks are not possible when using
    /// [`instantiate`](CellBuilder::instantiate).
    pub fn generate<I: PdkSchematic<PDK>>(&mut self, block: I) -> CellHandle<PDK::Primitive, I> {
        self.0.generate_pdk(block)
    }

    /// Generates a cell corresponding to `block` and returns a handle to it.
    ///
    /// Blocks on generation. Useful for handling errors thrown by the generation of a cell immediately.
    ///
    /// As with [`CellBuilder::generate`], the resulting handle must be added to the schematic with
    /// [`CellBuilder::add`] before it can be connected as an instance.
    pub fn generate_blocking<I: PdkSchematic<PDK>>(
        &mut self,
        block: I,
    ) -> Result<CellHandle<PDK::Primitive, I>> {
        self.0.generate_pdk_blocking(block)
    }

    /// Adds a cell generated with [`CellBuilder::generate`] to the current schematic.
    ///
    /// Does not block on generation. Spawns a thread that waits on the generation of
    /// the underlying cell and panics if generation fails. If error recovery is desired,
    /// check errors before calling this function using [`CellHandle::try_cell`].
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    pub fn add<I: PdkSchematic<PDK>>(
        &mut self,
        cell: CellHandle<PDK::Primitive, I>,
    ) -> Instance<PDK::Primitive, I> {
        self.0.add_pdk(cell)
    }

    /// Instantiate a schematic view of the given block.
    ///
    /// This function generates and adds the cell to the schematic. If checks need to be done on
    /// the generated cell before it is added to the schematic, use [`CellBuilder::generate`] and
    /// [`CellBuilder::add`].
    ///
    /// Spawns a thread that generates the underlying cell and panics if the generator fails. If error
    /// recovery is desired, use the generate and add workflow mentioned above.
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    pub fn instantiate<I: PdkSchematic<PDK>>(&mut self, block: I) -> Instance<PDK::Primitive, I> {
        self.instantiate(block)
    }

    /// Create an instance and immediately connect its ports.
    pub fn instantiate_connected<I, C>(&mut self, block: I, io: C)
    where
        I: PdkSchematic<PDK>,
        C: SchematicBundle,
        <I::Io as SchematicType>::Bundle: Connect<C>,
    {
        self.0.instantiate_pdk_connected(block, io)
    }
}

impl<PDK: Pdk, S: Schema> CellBuilder<PDK, S> {
    pub(crate) fn new(inner: CellBuilderInner<PDK, S::Primitive>) -> Self {
        Self(inner, PhantomData)
    }
    /// Create a new signal with the given name and hardware type.
    #[track_caller]
    pub fn signal<TY: SchematicType>(
        &mut self,
        name: impl Into<ArcStr>,
        ty: TY,
    ) -> <TY as SchematicType>::Bundle {
        self.0.signal(name, ty)
    }

    /// Connect all signals in the given data instances.
    pub fn connect<D1, D2>(&mut self, s1: D1, s2: D2)
    where
        D1: SchematicBundle,
        D2: SchematicBundle,
        D1: Connect<D2>,
    {
        self.0.connect(s1, s2)
    }

    /// Gets the global context.
    pub fn ctx(&self) -> &Context<PDK> {
        self.0.ctx()
    }

    /// Starts generating a block in a new thread and returns a handle to its cell.
    ///
    /// Can be used to check data stored in the cell or other generated results before adding the
    /// cell to the current schematic with [`CellBuilder::add`].
    ///
    /// To generate and add the block simultaneously, use [`CellBuilder::instantiate`]. However,
    /// error recovery and other checks are not possible when using
    /// [`instantiate`](CellBuilder::instantiate).
    pub fn generate<I: Schematic<PDK, S>>(&mut self, block: I) -> CellHandle<S::Primitive, I> {
        self.0.generate(block)
    }

    /// Generates a cell corresponding to `block` and returns a handle to it.
    ///
    /// Blocks on generation. Useful for handling errors thrown by the generation of a cell immediately.
    ///
    /// As with [`CellBuilder::generate`], the resulting handle must be added to the schematic with
    /// [`CellBuilder::add`] before it can be connected as an instance.
    pub fn generate_blocking<I: Schematic<PDK, S>>(
        &mut self,
        block: I,
    ) -> Result<CellHandle<S::Primitive, I>> {
        self.0.generate_blocking(block)
    }

    /// Adds a cell generated with [`CellBuilder::generate`] to the current schematic.
    ///
    /// Does not block on generation. Spawns a thread that waits on the generation of
    /// the underlying cell and panics if generation fails. If error recovery is desired,
    /// check errors before calling this function using [`CellHandle::try_cell`].
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    pub fn add<I: Schematic<PDK, S>>(
        &mut self,
        cell: CellHandle<S::Primitive, I>,
    ) -> Instance<S::Primitive, I> {
        self.0.add(cell)
    }

    /// Instantiate a schematic view of the given block.
    ///
    /// This function generates and adds the cell to the schematic. If checks need to be done on
    /// the generated cell before it is added to the schematic, use [`CellBuilder::generate`] and
    /// [`CellBuilder::add`].
    ///
    /// Spawns a thread that generates the underlying cell and panics if the generator fails. If error
    /// recovery is desired, use the generate and add workflow mentioned above.
    ///
    /// # Panics
    ///
    /// Immediately panics if this cell has been marked as a blackbox.
    /// A blackbox cell cannot contain instances or primitive devices.
    ///
    /// The spawned thread may panic after this function returns if cell generation fails.
    #[track_caller]
    pub fn instantiate<I: Schematic<PDK, S>>(&mut self, block: I) -> Instance<S::Primitive, I> {
        self.0.instantiate(block)
    }

    /// Create an instance and immediately connect its ports.
    pub fn instantiate_connected<I, C>(&mut self, block: I, io: C)
    where
        I: Schematic<PDK, S>,
        C: SchematicBundle,
        <I::Io as SchematicType>::Bundle: Connect<C>,
    {
        self.0.instantiate_connected(block, io)
    }
}

impl<PDK: ToSchema<S>, S: Schema> CellBuilder<PDK, S> {
    fn set_pdk_schematic<I: PdkSchematic<PDK>>(
        &mut self,
        io: &<<I as Block>::Io as SchematicType>::Bundle,
        block: &I,
    ) -> Result<I::NestedNodes>
    where
        PDK: ToSchema<S>,
    {
        let CellBuilderInner {
            ctx,
            id,
            cell_name,
            flatten,
            root,
            node_ctx,
            node_names,
            ports,
            ..
        } = self.0.clone_without_contents();
        let mut pdk_builder = PdkCellBuilder(CellBuilderInner {
            ctx,
            id,
            cell_name,
            flatten,
            root,
            node_ctx,
            node_names,
            ports,
            contents: CellBuilderContents::Cell(CellInner {
                next_instance_id: InstanceId(0),
                instances: Vec::new(),
            }),
        });
        let data = PdkSchematic::schematic(block, io, &mut pdk_builder)?;
        let CellBuilderInner {
            ctx,
            id,
            cell_name,
            flatten,
            root,
            node_ctx,
            node_names,
            ports,
            contents,
        } = pdk_builder.0;
        *self = CellBuilder::new(CellBuilderInner {
            ctx,
            id,
            cell_name,
            flatten,
            root,
            node_ctx,
            node_names,
            ports,
            contents: contents
                .convert_primitives(PDK::convert_primitive)
                .ok_or(Error::UnsupportedPrimitive)?,
        });
        Ok(data)
    }
}

/// A schematic cell.
pub struct Cell<P, T: ExportsNestedNodes> {
    /// The block from which this cell was generated.
    block: Arc<T>,
    /// Data returned by the cell's schematic generator.
    pub(crate) nodes: Arc<T::NestedNodes>,
    pub(crate) raw: Arc<RawCell<P>>,
    /// The cell's input/output interface.
    io: Arc<<T::Io as SchematicType>::Bundle>,
    /// The path corresponding to this cell.
    path: InstancePath,
}
impl<P, T: ExportsNestedNodes> Clone for Cell<P, T> {
    fn clone(&self) -> Self {
        Self {
            block: self.block.clone(),
            nodes: self.nodes.clone(),
            raw: self.raw.clone(),
            io: self.io.clone(),
            path: self.path.clone(),
        }
    }
}

impl<P, T: ExportsNestedNodes> Cell<P, T> {
    pub(crate) fn new(
        io: <T::Io as SchematicType>::Bundle,
        block: Arc<T>,
        data: T::NestedNodes,
        raw: Arc<RawCell<P>>,
    ) -> Self {
        let id = raw.id;
        Self {
            io: Arc::new(io),
            block,
            nodes: Arc::new(data),
            raw,
            path: InstancePath::new(id),
        }
    }

    /// Returns the block whose schematic this cell represents.
    pub fn block(&self) -> &T {
        &self.block
    }

    /// Returns nested nodes propagated by the cell's schematic generator.
    pub fn nodes(&self) -> NestedView<T::NestedNodes> {
        self.nodes.nested_view(&self.path)
    }

    /// Returns this cell's IO.
    pub fn io(&self) -> NestedView<<T::Io as SchematicType>::Bundle> {
        self.io.nested_view(&self.path)
    }
}

/// A handle to a schematic cell that is being generated.
pub struct CellHandle<P, T: ExportsNestedNodes> {
    pub(crate) id: CellId,
    pub(crate) block: Arc<T>,
    pub(crate) cell: CacheHandle<Result<Cell<P, T>>>,
}

impl<P, T: ExportsNestedNodes> Clone for CellHandle<P, T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            block: self.block.clone(),
            cell: self.cell.clone(),
        }
    }
}

impl<P, T: ExportsNestedNodes> CellHandle<P, T> {
    /// Tries to access the underlying [`Cell`].
    ///
    /// Blocks until cell generation completes and returns an error if one was thrown during generation.
    pub fn try_cell(&self) -> Result<&Cell<P, T>> {
        self.cell.try_inner().map_err(|e| match e {
            // TODO: Handle cache errors with more granularity.
            TryInnerError::CacheError(_) => Error::Internal,
            TryInnerError::GeneratorError(e) => e.clone(),
        })
    }

    /// Returns the underlying [`Cell`].
    ///
    /// Blocks until cell generation completes.
    ///
    /// # Panics
    ///
    /// Panics if generation fails.
    pub fn cell(&self) -> &Cell<P, T> {
        self.try_cell().expect("cell generation failed")
    }
}

/// An instance of a schematic cell.
#[allow(dead_code)]
pub struct Instance<P, T: ExportsNestedNodes> {
    id: InstanceId,
    /// Path of the parent cell.
    parent: InstancePath,
    /// Path to this instance relative to the current cell.
    path: InstancePath,
    /// The cell's input/output interface.
    io: <T::Io as SchematicType>::Bundle,
    cell: CellHandle<P, T>,
}

impl<P, T: ExportsNestedNodes> Instance<P, T> {
    /// The ports of this instance.
    ///
    /// Used for node connection purposes.
    pub fn io(&self) -> &<T::Io as SchematicType>::Bundle {
        &self.io
    }

    /// Tries to access this instance's terminals.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_terminals(&self) -> Result<TerminalView<<T::Io as SchematicType>::Bundle>> {
        self.cell
            .try_cell()
            .map(|cell| cell.io.terminal_view(&self.path))
    }

    /// Tries to access this instance's terminals
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn terminals(&self) -> TerminalView<<T::Io as SchematicType>::Bundle> {
        self.cell.cell().io.terminal_view(&self.path)
    }

    /// Tries to access the underlying [`Cell`].
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_data(&self) -> Result<InstanceData<T>> {
        self.cell.try_cell().map(|cell| InstanceData {
            block: cell.block.clone(),
            io: cell.io.clone(),
            nodes: cell.nodes.clone(),
            path: self.path.clone(),
        })
    }

    /// Returns the underlying [`Cell`].
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn data(&self) -> InstanceData<T> {
        self.try_data().expect("cell generation failed")
    }

    /// Tries to access the underlying cell data.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_nodes(&self) -> Result<NestedView<'_, T::NestedNodes>> {
        self.cell
            .try_cell()
            .map(|data| data.nodes.nested_view(&self.path))
    }

    /// Tries to access the underlying cell data.
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn nodes(&self) -> NestedView<'_, T::NestedNodes> {
        self.cell.cell().nodes.nested_view(&self.path)
    }

    /// Tries to access the underlying block used to create this instance's cell.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_block(&self) -> Result<&T> {
        self.cell.try_cell().map(|cell| cell.block.as_ref())
    }

    /// Tries to access the underlying block used to create this instance's cell.
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn block(&self) -> &T {
        &self.cell.cell().block
    }
}

/// A wrapper around schematic-specific context data.
#[derive(Debug, Clone)]
pub struct SchematicContext {
    next_id: CellId,
    pub(crate) cell_cache: TypeCache,
}

impl Default for SchematicContext {
    fn default() -> Self {
        Self {
            next_id: CellId(0),
            cell_cache: Default::default(),
        }
    }
}

impl SchematicContext {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn get_id(&mut self) -> CellId {
        self.next_id.increment();
        self.next_id
    }
}

/// A path to an instance from a top level cell.
///
/// Inexpensive to clone as it only clones an ID and a reference counted pointer.
#[derive(Debug, Clone)]
pub struct InstancePath {
    /// The ID of the top level cell that this path is relative to.
    pub(crate) top: CellId,
    /// The ID of the last instance's underlying cell.
    ///
    /// Allows for verification that two paths can be concatenated.
    /// `None` if path is empty.
    pub(crate) bot: Option<CellId>,
    /// A path of instance IDs.
    pub(crate) path: PathTree<InstanceId>,
}

impl InstancePath {
    pub(crate) fn new(top: CellId) -> Self {
        Self {
            top,
            bot: None,
            path: PathTree::empty(),
        }
    }
    #[allow(dead_code)]
    pub(crate) fn append(&self, other: &Self) -> Self {
        if let Some(bot) = self.bot {
            assert_eq!(
                bot, other.top,
                "path to append must start with the cell ID that the current path ends with"
            );
        } else {
            assert_eq!(
                self.top, other.top,
                "path to append must start with the cell ID that the current path ends with"
            );
        }
        Self {
            top: self.top,
            bot: other.bot,
            path: self.path.append(&other.path),
        }
    }

    pub(crate) fn prepend(&self, other: &Self) -> Self {
        if let Some(bot) = other.bot {
            assert_eq!(
                bot, self.top,
                "path to prepend must end with the cell ID that the current path starts with"
            );
        } else {
            assert_eq!(
                other.top, self.top,
                "path to prepend must end with the cell ID that the current path starts with"
            );
        }
        Self {
            top: other.top,
            bot: self.bot,
            path: self.path.prepend(&other.path),
        }
    }

    pub(crate) fn append_segment(&self, id: InstanceId, cell_id: CellId) -> Self {
        Self {
            top: self.top,
            bot: Some(cell_id),
            path: self.path.append_segment(id),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.bot.is_none()
    }
}

/// Data that can be stored in [`HasSchematicData::Data`](crate::schematic::ExportsNestedNodes::NestedNodes).
pub trait NestedNodes: HasNestedView + Send + Sync {}
impl<T: HasNestedView + Send + Sync> NestedNodes for T {}

pub struct InstanceData<B: ExportsNestedNodes> {
    block: Arc<B>,
    io: Arc<<<B as Block>::Io as SchematicType>::Bundle>,
    nodes: Arc<<B as ExportsNestedNodes>::NestedNodes>,
    path: InstancePath,
}

impl<B: ExportsNestedNodes> Clone for InstanceData<B> {
    fn clone(&self) -> Self {
        Self {
            block: self.block.clone(),
            io: self.io.clone(),
            nodes: self.nodes.clone(),
            path: self.path.clone(),
        }
    }
}

impl<B: ExportsNestedNodes> HasNestedView for InstanceData<B> {
    type NestedView<'a> = InstanceData<B>;
    fn nested_view(&self, parent: &InstancePath) -> Self::NestedView<'_> {
        let mut data = self.clone();
        data.path = self.path.prepend(parent);
        data
    }
}

impl<B: ExportsNestedNodes> InstanceData<B> {
    pub fn block(&self) -> &B {
        self.block.as_ref()
    }

    pub fn terminals(&self) -> TerminalView<<<B as Block>::Io as SchematicType>::Bundle> {
        self.io.terminal_view(&self.path)
    }

    pub fn nodes(&self) -> NestedView<<B as ExportsNestedNodes>::NestedNodes> {
        self.nodes.nested_view(&self.path)
    }

    pub fn path(&self) -> &InstancePath {
        &self.path
    }
}

/// An object that can be nested in the data of a cell.
///
/// Stores a path of instances up to the current cell using an [`InstancePath`].
pub trait HasNestedView {
    /// A view of the nested object.
    type NestedView<'a>
    where
        Self: 'a;

    /// Creates a nested view of the object given a parent node.
    fn nested_view(&self, parent: &InstancePath) -> Self::NestedView<'_>;
}

impl<T> HasNestedView for &T
where
    T: HasNestedView,
{
    type NestedView<'a>
    = T::NestedView<'a> where Self: 'a;

    fn nested_view(&self, parent: &InstancePath) -> Self::NestedView<'_> {
        (*self).nested_view(parent)
    }
}

// TODO: Potentially use lazy evaluation instead of cloning.
impl<T: HasNestedView> HasNestedView for Vec<T> {
    type NestedView<'a> = Vec<NestedView<'a, T>> where T: 'a;

    fn nested_view(&self, parent: &InstancePath) -> Self::NestedView<'_> {
        self.iter().map(|elem| elem.nested_view(parent)).collect()
    }
}

/// The associated nested view of an object.
pub type NestedView<'a, T> = <T as HasNestedView>::NestedView<'a>;

impl HasNestedView for () {
    type NestedView<'a> = ();

    fn nested_view(&self, _parent: &InstancePath) -> Self::NestedView<'_> {}
}

/// A raw (weakly-typed) instance of a cell.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct RawInstance<P> {
    id: InstanceId,
    name: ArcStr,
    child: Arc<RawCell<P>>,
    connections: Vec<Node>,
}

impl<P: Primitive> RawInstance<P> {
    fn convert_primitives<C>(self, convert_fn: fn(P) -> Option<C>) -> Option<RawInstance<C>> {
        let RawInstance {
            id,
            name,
            child,
            connections,
        } = self;
        Some(RawInstance {
            id,
            name,
            child: Arc::new((*child).clone().convert_primitives(convert_fn)?),
            connections,
        })
    }
}

/// A raw (weakly-typed) cell.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct RawCell<P> {
    id: CellId,
    name: ArcStr,
    ports: Vec<Port>,
    uf: NodeUf,
    node_names: HashMap<Node, NameBuf>,
    roots: HashMap<Node, Node>,
    contents: RawCellContents<P>,
    /// Whether this cell should be flattened when being exported.
    flatten: bool,
}

impl<P: Primitive> RawCell<P> {
    fn convert_primitives<C>(self, convert_fn: fn(P) -> Option<C>) -> Option<RawCell<C>> {
        let RawCell {
            id,
            name,
            ports,
            uf,
            node_names,
            roots,
            contents,
            flatten,
        } = self;
        Some(RawCell {
            id,
            name,
            ports,
            uf,
            node_names,
            roots,
            contents: contents.convert_primitives(convert_fn)?,
            flatten,
        })
    }
}

/// The contents of a raw cell.
pub(crate) type RawCellContents<P> =
    RawCellKind<RawCellInner<P>, ScirCellInner<P>, P, BlackboxContents>;

impl<P> From<CellBuilderContents<P>> for RawCellContents<P> {
    fn from(value: CellBuilderContents<P>) -> Self {
        match value {
            CellBuilderContents::Cell(CellInner { instances, .. }) => {
                RawCellContents::Cell(RawCellInner {
                    instances: instances
                        .into_iter()
                        .map(|instance| instance.recv().unwrap().unwrap())
                        .collect::<Vec<_>>(),
                })
            }
            CellBuilderContents::Scir(s) => RawCellContents::Scir(s),
            CellBuilderContents::Primitive(p) => RawCellContents::Primitive(p),
            CellBuilderContents::Blackbox(b) => RawCellContents::Blackbox(b),
        }
    }
}

impl<P: Primitive> RawCellContents<P> {
    fn convert_primitives<C>(self, convert_fn: fn(P) -> Option<C>) -> Option<RawCellContents<C>> {
        Some(match self {
            Self::Cell(c) => RawCellContents::Cell(c.convert_primitives(convert_fn)?),
            Self::Scir(c) => RawCellContents::Scir(c.convert_primitives(convert_fn)?),
            Self::Primitive(c) => RawCellContents::Primitive(convert_fn(c)?),
            Self::Blackbox(c) => RawCellContents::Blackbox(c),
        })
    }
}

/// An enumeration of raw cell kinds.
///
/// Can be used to store data associated with each kind of raw cell.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[enumify::enumify]
pub(crate) enum RawCellKind<C, S, P, B> {
    Cell(C),
    Scir(S),
    Primitive(P),
    Blackbox(B),
}

#[derive(Debug, Clone)]
pub(crate) struct RawCellInner<P> {
    instances: Vec<RawInstance<P>>,
}

impl<P: Primitive> RawCellInner<P> {
    fn convert_primitives<C>(self, convert_fn: fn(P) -> Option<C>) -> Option<RawCellInner<C>> {
        Some(RawCellInner {
            instances: self
                .instances
                .into_iter()
                .map(move |inst| inst.convert_primitives(convert_fn))
                .collect::<Option<Vec<_>>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ScirCellInner<P> {
    pub(crate) lib: Arc<scir::Library<P>>,
    pub(crate) cell: scir::CellId,
}

impl<P: Primitive> ScirCellInner<P> {
    fn convert_primitives<C>(self, convert_fn: fn(P) -> Option<C>) -> Option<ScirCellInner<C>> {
        let ScirCellInner { lib, cell } = self;
        Some(ScirCellInner {
            lib: Arc::new((*lib).clone().convert_primitives(convert_fn)?),
            cell,
        })
    }
}

/// A context-wide unique identifier for a cell.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CellId(u64);

impl CellId {
    pub(crate) fn increment(&mut self) {
        *self = CellId(self.0 + 1)
    }
}

/// A cell-wide unique identifier for an instance.
#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct InstanceId(pub(crate) u64);

impl InstanceId {
    pub(crate) fn increment(&mut self) {
        *self = InstanceId(self.0 + 1)
    }
}
