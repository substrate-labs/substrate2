use std::collections::HashMap;

use geometry::{dir::Dir, span::Span};
use grid::Grid;

pub struct CellId(u64);

pub struct Cell {
    dir: Dir,
    span: Span,
    tracks: Vec<Span>,
}

impl Cell {
    // Half tracks are allowed though?
    pub fn validate(&self) -> bool {
        for track in self.tracks.iter() {
            if track.stop() < self.span.start() || track.start() > self.span.stop() {
                return false;
            }
        }
        true
    }
}

/// The set of cells on a single layer.
pub struct LayerCells {
    cells: HashMap<CellId, Cell>,
    grid: Grid<Option<CellId>>,
}

pub struct PdkLayer {
    dir: Dir,
    primitive_cell_width: i64,
}

pub struct Pdk {
    primitive_cell_x: i64,
    primitive_cell_y: i64,
    routing_layers: Vec<PdkLayer>,
}

impl Pdk {
    fn validate(&self) -> bool {
        let mut prev_dir = None;
        for layer in &self.routing_layers {
            if let Some(prev_dir) = prev_dir {
                if prev_dir == layer.dir {
                    return false;
                }
            }
            prev_dir = Some(layer.dir);
            if layer.primitive_cell_width
                % match layer.dir {
                    Dir::Horiz => self.primitive_cell_y,
                    Dir::Vert => self.primitive_cell_x,
                }
                != 0
            {
                return false;
            }
        }
        return true;
    }
}
