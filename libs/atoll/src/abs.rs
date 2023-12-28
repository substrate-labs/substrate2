//! Generate abstract views of layout cells.
use crate::grid::{LayerSlice, LayerStack, PdkLayer, RoutingGrid, RoutingState};
use crate::PointState;
use grid::Grid;
use num::integer::{div_ceil, div_floor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use substrate::arcstr::ArcStr;
use substrate::block::Block;
use substrate::geometry::bbox::Bbox;

use substrate::geometry::point::Point;
use substrate::geometry::rect::Rect;
use substrate::geometry::transform::Transformation;
use substrate::io::layout::Builder;
use substrate::layout::element::{CellId, Element, RawCell};
use substrate::layout::element::{Shape, Text};

use substrate::layout::{CellBuilder, Draw, DrawReceiver, ExportsLayoutData, Layout};
use substrate::pdk::layers::HasPin;
use substrate::pdk::Pdk;
use substrate::schematic::ExportsNestedData;
use substrate::{arcstr, layout};

/// The abstract view of an ATOLL tile.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AtollAbstract {
    /// The topmost ATOLL layer used within the tile.
    top_layer: usize,
    /// The bounds of the tile, in LCM units with respect to `top_layer`.
    ///
    /// The "origin" of the tile, in LCM units, is the lower left of this rectangle.
    lcm_bounds: Rect,
    /// The state of each layer, up to and including `top_layer`.
    ///
    /// Ports on layers not supported by ATOLL are ignored.
    layers: Vec<LayerAbstract>,
    /// The routing grid used to produce this abstract view.
    pub(crate) grid: RoutingGrid<PdkLayer>,
}

impl AtollAbstract {
    pub fn physical_bounds(&self) -> Rect {
        let slice = self.slice();
        let w = slice.lcm_unit_width();
        let h = slice.lcm_unit_height();
        Rect::from_sides(
            self.lcm_bounds.left() * w,
            self.lcm_bounds.bot() * h,
            self.lcm_bounds.right() * w,
            self.lcm_bounds.top() * h,
        )
    }

    fn slice(&self) -> LayerSlice<'_, PdkLayer> {
        self.grid.slice()
    }

    pub fn physical_origin(&self) -> Point {
        self.lcm_bounds.lower_left() * self.slice().lcm_units()
    }
}

/// The abstracted state of a single routing layer.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum LayerAbstract {
    /// The layer is fully blocked.
    ///
    /// No routing on this layer is permitted.
    Blocked,
    /// The layer is available for routing and exposes the state of each point on the routing grid.
    Detailed { states: Grid<PointState> },
}

fn top_layer(cell: &RawCell, stack: &LayerStack<PdkLayer>) -> Option<usize> {
    let mut state = HashMap::new();
    top_layer_inner(cell, &mut state, stack)
}

fn top_layer_inner(
    cell: &RawCell,
    state: &mut HashMap<CellId, Option<usize>>,
    stack: &LayerStack<PdkLayer>,
) -> Option<usize> {
    if let Some(&layer) = state.get(&cell.id()) {
        return layer;
    }

    let mut top = None;

    for elt in cell.elements() {
        match elt {
            Element::Instance(inst) => {
                top = top.max(top_layer_inner(inst.raw_cell(), state, stack));
            }
            Element::Shape(s) => {
                if let Some(layer) = stack.layer_idx(s.layer()) {
                    top = top.max(Some(layer));
                }
            }
            Element::Text(_) => {
                // ignore text elements for the sake of calculating top layers
            }
        }
    }

    state.insert(cell.id(), top);
    top
}

pub fn generate_abstract<T: ExportsNestedData + ExportsLayoutData>(
    layout: &layout::Cell<T>,
    stack: &LayerStack<PdkLayer>,
) -> AtollAbstract {
    let cell = layout.raw();
    let bbox = cell.bbox().unwrap();

    let top = top_layer(cell, stack)
        .expect("cell did not have any ATOLL routing layers; cannot produce an abstract");
    let top = if top == 0 { 1 } else { top };

    let slice = stack.slice(0..top + 1);

    let xmin = div_floor(bbox.left(), slice.lcm_unit_width());
    let xmax = div_ceil(bbox.right(), slice.lcm_unit_width());
    let ymin = div_floor(bbox.bot(), slice.lcm_unit_height());
    let ymax = div_ceil(bbox.top(), slice.lcm_unit_height());
    let lcm_bounds = Rect::from_sides(xmin, ymin, xmax, ymax);

    let nx = lcm_bounds.width();
    let ny = lcm_bounds.height();

    let grid = RoutingGrid::new(stack.clone(), 0..top + 1, nx, ny);
    let mut state = RoutingState::new(stack.clone(), top, nx, ny);
    for (name, geom) in cell.ports() {
        if let Some(layer) = stack.layer_idx(geom.primary.layer().drawing()) {
            let rect = match geom.primary.shape() {
                substrate::geometry::shape::Shape::Rect(r) => *r,
                substrate::geometry::shape::Shape::Polygon(p) => {
                    p.bbox().expect("empty polygons are unsupported")
                }
            };
            println!("source rect = {rect:?}");
            if let Some(rect) = grid.shrink_to_grid(rect, layer) {
                println!("grid rect = {rect:?}");
                for x in rect.left()..=rect.right() {
                    for y in rect.bot()..=rect.top() {
                        let xofs = xmin * slice.lcm_unit_width() / grid.xpitch(layer);
                        let yofs = ymin * slice.lcm_unit_height() / grid.ypitch(layer);
                        println!("assigning ({}, {}) to net {}", x - xofs, y - yofs, name);
                        state.layer_mut(layer)[((x - xofs) as usize, (y - yofs) as usize)] =
                            PointState::Obstructed;
                    }
                }
            }
        }
    }

    let layers = state
        .layers
        .into_iter()
        .map(|states| LayerAbstract::Detailed { states })
        .collect();

    AtollAbstract {
        top_layer: top,
        lcm_bounds,
        grid: RoutingGrid::new(stack.clone(), 0..top + 1, nx, ny),
        layers,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DebugAbstract {
    pub abs: AtollAbstract,
    pub stack: LayerStack<PdkLayer>,
}

impl Block for DebugAbstract {
    type Io = ();
    fn id() -> ArcStr {
        arcstr::literal!("debug_abstract")
    }
    fn name(&self) -> ArcStr {
        Self::id()
    }
    fn io(&self) -> Self::Io {
        Default::default()
    }
}

impl ExportsLayoutData for DebugAbstract {
    type LayoutData = ();
}

impl<PDK: Pdk> Layout<PDK> for DebugAbstract {
    #[inline]
    fn layout(
        &self,
        _io: &mut Builder<<Self as Block>::Io>,
        cell: &mut CellBuilder<PDK, Self>,
    ) -> substrate::error::Result<Self::LayoutData> {
        cell.draw(self)?;
        Ok(())
    }
}

impl<PDK: Pdk> Draw<PDK> for &DebugAbstract {
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> substrate::error::Result<()> {
        for (i, layer) in self.abs.layers.iter().enumerate() {
            let layer_id = self.abs.grid.stack.layer(i).id;
            match layer {
                LayerAbstract::Blocked => {
                    recv.draw(Shape::new(layer_id, self.abs.physical_bounds()))?;
                }
                LayerAbstract::Detailed { states } => {
                    let (tx, ty) = states.size();
                    let xofs = self.abs.lcm_bounds.left() * self.abs.slice().lcm_unit_width()
                        / self.abs.grid.xpitch(i);
                    let yofs = self.abs.lcm_bounds.bot() * self.abs.slice().lcm_unit_height()
                        / self.abs.grid.ypitch(i);
                    for x in 0..tx {
                        for y in 0..ty {
                            let pt =
                                self.abs
                                    .grid
                                    .xy_track_point(i, x as i64 + xofs, y as i64 + yofs);
                            let rect = match states[(x, y)] {
                                PointState::Available => Rect::from_point(pt).expand_all(20),
                                PointState::Obstructed => Rect::from_point(pt).expand_all(40),
                                PointState::Routed { .. } => Rect::from_point(pt).expand_all(30),
                            };
                            recv.draw(Shape::new(layer_id, rect))?;
                            let text = Text::new(
                                layer_id,
                                format!("({x},{y})"),
                                Transformation::translate(pt.x as f64, pt.y as f64),
                            );
                            recv.draw(text)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl<PDK: Pdk> Draw<PDK> for DebugAbstract {
    #[inline]
    fn draw(self, recv: &mut DrawReceiver<PDK>) -> substrate::error::Result<()> {
        recv.draw(&self)
    }
}
