//! Substrate's layout generator framework.
//!
//! # Examples
//!
//! ## Simple
#![doc = examples::get_snippets!("core", "inverter_layout")]
//!
//! ## With data
#![doc = examples::get_snippets!("core", "buffer_layout")]

use std::{marker::PhantomData, sync::Arc, thread};

use arcstr::ArcStr;
use cache::{error::TryInnerError, mem::TypeCache, CacheHandle};
pub use codegen::{Layout, LayoutData};
use examples::get_snippets;
use geometry::{
    prelude::{Bbox, Point},
    transform::{
        HasTransformedView, Transform, TransformMut, Transformation, Transformed, TranslateMut,
    },
    union::BoundingUnion,
};
use once_cell::sync::OnceCell;

use crate::io::LayoutType;
use crate::pdk::{HasLayout, Pdk};
use crate::sealed;
use crate::{block::Block, error::Error};
use crate::{context::Context, error::Result};

use self::element::{CellId, Element, RawCell, RawInstance, Shape};

pub mod element;
pub mod error;
pub mod gds;
pub mod tiling;

/// Data exported from a generated layout.
///
/// Contained data is transformed with the containing instance
/// according to its [`HasTransformedView`] implementation.
pub trait LayoutData: HasTransformedView + Send + Sync {}
impl<T: HasTransformedView + Send + Sync> LayoutData for T {}

/// A block that exports data from its layout.
///
/// All blocks that have a layout implementation must export data.
pub trait ExportsLayoutData: Block {
    /// Extra layout data to be stored with the block's generated cell.
    ///
    /// When the block is instantiated and transformed, all contained data
    /// will be transformed with the block.
    type Data: LayoutData;
}

/// A block that can be laid out in process design kit `PDK`.
pub trait Layout<PDK: Pdk>: ExportsLayoutData {
    /// Generates the block's layout.
    fn layout(
        &self,
        io: &mut <<Self as Block>::Io as LayoutType>::Builder,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> Result<Self::Data>;
}

/// A block that implements [`Layout<PDK>`].
///
/// Automatically implemented for blocks that implement [`Layout<PDK>`] and
/// cannot be implemented outside of Substrate.
pub trait LayoutImplemented<PDK: Pdk>: ExportsLayoutData {
    /// Generates the block's layout.
    ///
    /// For internal use only.
    #[doc(hidden)]
    fn layout_impl(
        &self,
        io: &mut <<Self as Block>::Io as LayoutType>::Builder,
        cell: &mut CellBuilder<PDK, Self>,
        _: sealed::Token,
    ) -> Result<Self::Data>;
}

impl<PDK: Pdk, B: ExportsLayoutData> LayoutImplemented<PDK> for B
where
    PDK: HasLayout<B>,
{
    fn layout_impl(
        &self,
        io: &mut <<Self as Block>::Io as LayoutType>::Builder,
        cell: &mut CellBuilder<PDK, Self>,
        _: sealed::Token,
    ) -> Result<Self::Data> {
        PDK::layout(self, io, cell, sealed::Token)
    }
}

/// Layout-specific context data.
///
/// Stores generated layout cells as well as state used for assigning unique cell IDs.
#[derive(Debug, Default, Clone)]
pub struct LayoutContext {
    next_id: CellId,
    pub(crate) cell_cache: TypeCache,
}

impl LayoutContext {
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn get_id(&mut self) -> CellId {
        self.next_id.increment();
        self.next_id
    }
}

/// A generic layout cell.
///
/// Stores its underlying block, extra data created during generation, as well as a raw cell
/// containing its primitive elements.
///
/// # Examples
///
#[doc = get_snippets!("core", "generate")]
#[derive(Clone)]
#[allow(dead_code)]
pub struct Cell<T: ExportsLayoutData> {
    /// Block whose layout this cell represents.
    block: Arc<T>,
    /// Extra data created during layout generation.
    data: T::Data,
    pub(crate) io: Arc<<T::Io as LayoutType>::Bundle>,
    pub(crate) raw: Arc<RawCell>,
}

impl<T: ExportsLayoutData> Cell<T> {
    pub(crate) fn new(
        block: Arc<T>,
        data: T::Data,
        io: Arc<<T::Io as LayoutType>::Bundle>,
        raw: Arc<RawCell>,
    ) -> Self {
        Self {
            block,
            data,
            io,
            raw,
        }
    }

    /// Returns the block whose layout this cell represents.
    pub fn block(&self) -> &T {
        &self.block
    }

    /// Returns extra data created by the cell's schematic generator.
    pub fn data(&self) -> &T::Data {
        &self.data
    }

    /// Returns the geometry of the cell's IO.
    pub fn io(&self) -> &<T::Io as LayoutType>::Bundle {
        self.io.as_ref()
    }
}

impl<T: ExportsLayoutData> Bbox for Cell<T> {
    fn bbox(&self) -> Option<geometry::rect::Rect> {
        self.raw.bbox()
    }
}

/// A handle to a schematic cell that is being generated.
pub struct CellHandle<T: ExportsLayoutData> {
    pub(crate) cell: CacheHandle<Result<Cell<T>>>,
}

impl<T: ExportsLayoutData> Clone for CellHandle<T> {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell.clone(),
        }
    }
}

impl<T: ExportsLayoutData> CellHandle<T> {
    /// Tries to access the underlying [`Cell`].
    ///
    /// Blocks until cell generation completes and returns an error if one was thrown during generation.
    pub fn try_cell(&self) -> Result<&Cell<T>> {
        self.cell.try_inner().map_err(|e| match e {
            TryInnerError::CacheError(e) => Error::CacheError(e.clone()),
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
    pub fn cell(&self) -> &Cell<T> {
        self.try_cell().expect("cell generation failed")
    }
}

/// A transformed view of a cell, usually created by accessing the cell of an instance.
pub struct TransformedCell<'a, T: ExportsLayoutData> {
    /// Block whose layout this cell represents.
    block: &'a T,
    /// Extra data created during layout generation.
    data: Transformed<'a, T::Data>,
    /// The geometry of the cell's IO.
    io: Transformed<'a, <T::Io as LayoutType>::Bundle>,
    pub(crate) raw: Arc<RawCell>,
    pub(crate) trans: Transformation,
}

impl<'a, T: ExportsLayoutData> TransformedCell<'a, T> {
    /// Returns the block whose layout this cell represents.
    pub fn block(&self) -> &T {
        self.block
    }

    /// Returns extra data created by the cell's schematic generator.
    pub fn data(&'a self) -> &Transformed<'a, T::Data> {
        &self.data
    }
}

impl<T: ExportsLayoutData> HasTransformedView for Cell<T> {
    type TransformedView<'a> = TransformedCell<'a, T>;

    fn transformed_view(&self, trans: Transformation) -> Self::TransformedView<'_> {
        Self::TransformedView {
            block: &self.block,
            data: self.data.transformed_view(trans),
            io: self.io.transformed_view(trans),
            raw: self.raw.clone(),
            trans,
        }
    }
}

impl<'a, T: ExportsLayoutData> Bbox for TransformedCell<'a, T> {
    fn bbox(&self) -> Option<geometry::rect::Rect> {
        self.raw.bbox().transform(self.trans)
    }
}

/// A generic layout instance.
///
/// Stores a pointer to its underlying cell and its instantiated transformation.
#[allow(dead_code)]
pub struct Instance<T: ExportsLayoutData> {
    cell: CellHandle<T>,
    pub(crate) trans: Transformation,
}

impl<T: ExportsLayoutData> Clone for Instance<T> {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell.clone(),
            ..*self
        }
    }
}

impl<T: ExportsLayoutData> Instance<T> {
    pub(crate) fn new(cell: CellHandle<T>) -> Self {
        Instance {
            cell,
            trans: Transformation::default(),
        }
    }

    /// Tries to access a transformed view of the underlying [`Cell`], blocking on generation.
    ///
    /// Blocks until cell generation completes.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_cell(&self) -> Result<Transformed<'_, Cell<T>>> {
        self.cell
            .try_cell()
            .map(|cell| cell.transformed_view(self.trans))
    }

    /// Returns a transformed view of the underlying [`Cell`].
    ///
    /// Blocks until cell generation completes.
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn cell(&self) -> Transformed<'_, Cell<T>> {
        self.try_cell().expect("cell generation failed")
    }

    /// Tries to access extra data created by the cell's schematic generator.
    ///
    /// Blocks until cell generation completes.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_data(&self) -> Result<Transformed<'_, T::Data>> {
        Ok(self.try_cell()?.data)
    }

    /// Tries to access extra data created by the cell's schematic generator.
    ///
    /// Blocks until cell generation completes.
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn data(&self) -> Transformed<'_, T::Data> {
        self.cell().data
    }

    /// Tries to access the underlying block used to create this instance's cell.
    ///
    /// Blocks until cell generation completes.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_block(&self) -> Result<&T> {
        Ok(self.try_cell()?.block)
    }

    /// Tries to access the underlying block used to create this instance's cell.
    ///
    /// Blocks until cell generation completes.
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn block(&self) -> &T {
        self.cell().block
    }

    /// Returns a transformed view of the underlying [`Cell`]'s IO.
    ///
    /// Blocks until cell generation completes.
    ///
    /// Returns an error if one was thrown during generation.
    pub fn try_io(&self) -> Result<Transformed<'_, <T::Io as LayoutType>::Bundle>> {
        Ok(self.try_cell()?.io)
    }

    /// Returns a transformed view of the underlying [`Cell`]'s IO.
    ///
    /// Blocks until cell generation completes.
    ///
    /// # Panics
    ///
    /// Panics if an error was thrown during generation.
    pub fn io(&self) -> Transformed<'_, <T::Io as LayoutType>::Bundle> {
        self.cell().io
    }
}

impl<T: ExportsLayoutData> Bbox for Instance<T> {
    fn bbox(&self) -> Option<geometry::rect::Rect> {
        self.cell().bbox()
    }
}

impl<T: ExportsLayoutData> TranslateMut for Instance<T> {
    fn translate_mut(&mut self, p: Point) {
        self.transform_mut(Transformation::from_offset(p))
    }
}

impl<T: ExportsLayoutData> TransformMut for Instance<T> {
    fn transform_mut(&mut self, trans: Transformation) {
        self.trans = Transformation::cascade(trans, self.trans);
    }
}

impl<T: ExportsLayoutData> HasTransformedView for Instance<T> {
    type TransformedView<'a> = Instance<T>;

    fn transformed_view(&self, trans: Transformation) -> Self::TransformedView<'_> {
        self.clone().transform(trans)
    }
}

impl<PDK: Pdk, I: LayoutImplemented<PDK>> Draw<PDK> for Instance<I> {
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> Result<()> {
        recv.draw_instance(self);
        Ok(())
    }
}

/// A layout cell builder.
///
/// Constructed once for each invocation of [`Layout::layout`].
pub struct CellBuilder<PDK: Pdk, T> {
    phantom: PhantomData<T>,
    container: Container<PDK>,
    /// The current global context.
    pub ctx: Context<PDK>,
}

impl<PDK: Pdk, T> CellBuilder<PDK, T> {
    pub(crate) fn new(ctx: Context<PDK>) -> Self {
        Self {
            phantom: PhantomData,
            container: Container::new(),
            ctx,
        }
    }

    pub(crate) fn finish(self, id: CellId, name: ArcStr) -> RawCell {
        let mut cell = RawCell::new(id, name);

        self.container
            .finish(&mut cell.elements, &mut cell.blockages);

        cell
    }

    /// Generate an instance of `block`.
    ///
    /// Returns immediately, allowing generation to complete in the background. Attempting to
    /// access the generated instance's cell will block until generation is complete.
    ///
    /// # Examples
    ///
    #[doc = get_snippets!("core", "cell_builder_generate")]
    pub fn generate<I: LayoutImplemented<PDK>>(&mut self, block: I) -> Instance<I> {
        let cell = self.ctx.generate_layout(block);
        Instance::new(cell)
    }

    /// Generate an instance of `block`.
    ///
    /// Blocks on generation, returning only once the instance's cell is populated. Useful for
    /// handling errors thrown by the generation of a cell immediately.
    pub fn generate_blocking<I: LayoutImplemented<PDK>>(
        &mut self,
        block: I,
    ) -> Result<Instance<I>> {
        let cell = self.ctx.generate_layout(block);
        cell.try_cell()?;
        Ok(Instance::new(cell))
    }

    /// Draw a blockage.
    pub fn draw_blockage(&mut self, shape: Shape) {
        self.container.draw_blockage(shape)
    }

    /// Draw layout object `obj`.
    ///
    /// For instances, a new thread is spawned to add the instance once the underlying cell has
    /// been generated. If generation fails, the spawned thread may panic after this function has
    /// been called.
    ///
    /// For error recovery, instance generation results should be checked using [`Instance::try_cell`]
    /// before calling `draw`.
    ///
    /// # Panics
    ///
    /// May cause a panic if generation of an underlying instance fails.
    pub fn draw(&mut self, obj: impl Draw<PDK>) -> Result<()> {
        Container::draw(&mut self.container, obj)
    }

    /// Gets the global context.
    pub fn ctx(&self) -> &Context<PDK> {
        &self.ctx
    }
}

impl<PDK: Pdk, T> Bbox for CellBuilder<PDK, T> {
    fn bbox(&self) -> Option<geometry::rect::Rect> {
        self.container.bbox()
    }
}

/// A receiver for drawing layout objects.
///
/// Implements the primitive functions that layout objects need to implement [`Draw`].
#[derive(Debug, Clone)]
pub struct DrawReceiver<PDK> {
    phantom: PhantomData<PDK>,
    containers: Vec<Container<PDK>>,
    instances: Vec<Arc<OnceCell<Option<RawInstance>>>>,
    elements: Vec<Element>,
    blockages: Vec<Shape>,
    trans: Transformation,
}

impl<PDK> DrawReceiver<PDK> {
    pub(crate) fn new() -> Self {
        Self {
            phantom: PhantomData,
            containers: Vec::new(),
            instances: Vec::new(),
            elements: Vec::new(),
            blockages: Vec::new(),
            trans: Transformation::default(),
        }
    }

    /// Blocks on instances and returns pointers to them.
    fn get_instances(&self) -> Vec<&RawInstance> {
        self.instances
            .iter()
            .map(|instance| instance.wait().as_ref().unwrap())
            .collect()
    }

    pub(crate) fn finish(self, elements: &mut Vec<Element>, blockages: &mut Vec<Shape>) {
        for instance in self
            .instances
            .into_iter()
            .map(|instance| instance.wait().clone().unwrap())
        {
            elements.push(instance.transform(self.trans).into());
        }

        elements.extend(
            self.elements
                .into_iter()
                .map(|element| element.transform(self.trans)),
        );
        blockages.extend(
            self.blockages
                .into_iter()
                .map(|blockage| blockage.transform(self.trans)),
        );

        for mut container in self.containers {
            container.transform_mut(self.trans);
            container.finish(elements, blockages);
        }
    }

    pub(crate) fn draw_container(&mut self, container: Container<PDK>) {
        self.containers.push(container);
    }

    pub(crate) fn draw_element(&mut self, element: impl Into<Element>) {
        let element = element.into();
        self.elements.push(element);
    }

    /// Draw a blockage.
    pub fn draw_blockage(&mut self, shape: impl Into<Shape>) {
        self.blockages.push(shape.into());
    }
}

impl<PDK: Pdk> DrawReceiver<PDK> {
    pub(crate) fn draw_instance<I: LayoutImplemented<PDK>>(&mut self, inst: Instance<I>) {
        let instance = Arc::new(OnceCell::new());
        self.instances.push(instance.clone());

        let cell = inst.cell.clone();
        thread::spawn(move || {
            instance.set(cell.try_cell().ok().map(|cell| RawInstance {
                cell: cell.raw.clone(),
                trans: inst.trans,
            }))
        });
    }

    /// Draw layout object `obj`.
    ///
    /// For instances, a new thread is spawned to add the instance once the underlying cell has
    /// been generated. If generation fails, the spawned thread may panic after this function has
    /// been called.
    ///
    /// For error recovery, instance generation results should be checked using [`Instance::try_cell`]
    /// before calling `draw`.
    ///
    /// # Panics
    ///
    /// May cause a panic if generation of an underlying instance fails.
    pub fn draw(&mut self, obj: impl Draw<PDK>) -> Result<()> {
        obj.draw(self)
    }
}

impl<PDK: Pdk> Draw<PDK> for DrawReceiver<PDK> {
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> Result<()> {
        recv.containers.extend(self.containers);
        recv.instances.extend(self.instances);
        recv.elements.extend(self.elements);
        recv.blockages.extend(self.blockages);
        Ok(())
    }
}

impl<PDK> Bbox for DrawReceiver<PDK> {
    fn bbox(&self) -> Option<geometry::rect::Rect> {
        self.get_instances()
            .bbox()
            .bounding_union(&self.elements.bbox())
    }
}

/// An object that can be drawn in a [`CellBuilder`].
pub trait Draw<PDK: Pdk>: DrawBoxed<PDK> {
    /// Draws `self` inside `recv`.
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> Result<()>;
}

/// An object where `Box<Self>` can be drawn.
pub trait DrawBoxed<PDK: Pdk> {
    /// Draws `self` inside `recv`.
    fn draw_boxed(self: Box<Self>, recv: &mut DrawReceiver<PDK>) -> Result<()>;
}

impl<PDK: Pdk, T: Draw<PDK>> DrawBoxed<PDK> for T {
    fn draw_boxed(self: Box<Self>, recv: &mut DrawReceiver<PDK>) -> Result<()> {
        (*self).draw(recv)
    }
}

pub(crate) trait DrawContainer<PDK: Pdk>: Draw<PDK> {
    /// Draws `self` into a new [`Container`].
    fn draw_container(self) -> Result<Container<PDK>>;
}

impl<PDK: Pdk, T: Draw<PDK>> DrawContainer<PDK> for T {
    fn draw_container(self) -> Result<Container<PDK>> {
        let mut container = Container::new();
        Container::draw(&mut container, self)?;
        Ok(container)
    }
}

impl<PDK: Pdk, T: Draw<PDK> + ?Sized> Draw<PDK> for Box<T> {
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> Result<()> {
        self.draw_boxed(recv)
    }
}

/// TODO: Temporarily private until we decide whether it is worth exposing.
#[derive(Debug, Clone)]
pub(crate) struct Container<PDK> {
    recvs: Vec<DrawReceiver<PDK>>,
    trans: Transformation,
}

impl<PDK> Default for Container<PDK> {
    fn default() -> Self {
        Self {
            recvs: vec![DrawReceiver::new()],
            trans: Transformation::default(),
        }
    }
}

impl<PDK> Container<PDK> {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn finish(self, elements: &mut Vec<Element>, blockages: &mut Vec<Shape>) {
        for mut recv in self.recvs {
            recv.trans = Transformation::cascade(self.trans, recv.trans);
            recv.finish(elements, blockages);
        }
    }

    pub(crate) fn recv_mut(&mut self) -> &mut DrawReceiver<PDK> {
        self.recvs.last_mut().unwrap()
    }

    /// Draw a blockage.
    pub fn draw_blockage(&mut self, shape: impl Into<Shape>) {
        self.recv_mut().draw_blockage(shape);
    }
}

impl<PDK: Pdk> Container<PDK> {
    /// Draw layout object `obj`.
    ///
    /// For instances, a new thread is spawned to add the instance once the underlying cell has
    /// been generated. If generation fails, the spawned thread may panic after this function has
    /// been called.
    ///
    /// For error recovery, instance generation results should be checked using [`Instance::try_cell`]
    /// before calling `draw`.
    ///
    /// # Panics
    ///
    /// May cause a panic if generation of an underlying instance fails.
    pub fn draw(&mut self, obj: impl Draw<PDK>) -> Result<()> {
        self.recv_mut().draw(obj)
    }
}

impl<PDK> Bbox for Container<PDK> {
    fn bbox(&self) -> Option<geometry::rect::Rect> {
        self.recvs.bbox().transform(self.trans)
    }
}

impl<PDK: Pdk> Draw<PDK> for Container<PDK> {
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> Result<()> {
        recv.draw_container(self);
        Ok(())
    }
}

impl<PDK> TranslateMut for Container<PDK> {
    fn translate_mut(&mut self, p: Point) {
        self.transform_mut(Transformation::from_offset(p))
    }
}

impl<PDK> TransformMut for Container<PDK> {
    fn transform_mut(&mut self, trans: Transformation) {
        self.trans = Transformation::cascade(trans, self.trans);
        let mut recv = DrawReceiver::new();
        recv.trans = self.trans.inv();
        self.recvs.push(recv);
    }
}
