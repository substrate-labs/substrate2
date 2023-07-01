//! Utilities for GDS conversion.
//!
//! Converts between Substrate's layout data-model and [`gds`] structures.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use arcstr::ArcStr;
use geometry::{
    prelude::{Corner, NamedOrientation, Orientation, Point},
    rect::Rect,
};
use tracing::{span, Level};

use crate::{
    io::{IoShape, NameBuf, PortGeometry},
    pdk::layers::{GdsLayerSpec, HasPin, LayerContext, LayerId},
};

use super::{
    element::{CellId, Element, RawCell, RawInstance, Shape, Text},
    error::GdsExportResult,
};

/// An exporter for GDS files.
///
/// Takes a [`RawCell`] and converts it to a [`gds::GdsLibrary`].
pub struct GdsExporter<'a> {
    cell: Arc<RawCell>,
    layers: &'a LayerContext,
    cell_db: CellDb,
    gds: gds::GdsLibrary,
}

#[derive(Default)]
struct CellDb {
    names: HashSet<ArcStr>,
    assignments: HashMap<CellId, ArcStr>,
}

impl<'a> GdsExporter<'a> {
    /// Creates a new GDS exporter.
    ///
    /// Requires the cell to be exported and a [`LayerContext`] for mapping Substrate layers to GDS
    /// layers.
    pub fn new(cell: Arc<RawCell>, layers: &'a LayerContext) -> Self {
        Self {
            cell,
            layers,
            cell_db: Default::default(),
            gds: gds::GdsLibrary::new("TOP"),
        }
    }

    /// Exports the contents of `self` as a [`gds::GdsLibrary`].
    pub fn export(mut self) -> GdsExportResult<gds::GdsLibrary> {
        self.cell.clone().export(&mut self)?;
        Ok(self.gds)
    }

    fn get_name(&self, cell: &RawCell) -> Option<ArcStr> {
        self.cell_db.get_name(cell)
    }

    fn assign_name(&mut self, cell: &RawCell) -> ArcStr {
        self.cell_db.assign_name(cell)
    }

    fn get_layer(&self, id: LayerId) -> Option<GdsLayerSpec> {
        self.layers.get_gds_layer_from_id(id)
    }
}

impl CellDb {
    /// Returns whether the cell has already been exported.
    fn get_name(&self, cell: &RawCell) -> Option<ArcStr> {
        self.assignments.get(&cell.id).cloned()
    }

    /// Returns a new name if th cell needs to be generated.
    fn assign_name(&mut self, cell: &RawCell) -> ArcStr {
        let name = &cell.name;
        let name = if self.names.contains(name) {
            let mut i = 1;
            loop {
                let new_name = arcstr::format!("{}_{}", name, i);
                if !self.names.contains(&new_name) {
                    break new_name;
                }
                i += 1;
            }
        } else {
            name.clone()
        };

        self.names.insert(name.clone());
        self.assignments.insert(cell.id, name.clone());
        name
    }
}

#[allow(clippy::from_over_into)]
impl Into<gds::GdsLayerSpec> for GdsLayerSpec {
    fn into(self) -> gds::GdsLayerSpec {
        gds::GdsLayerSpec {
            layer: self.0 as i16,
            xtype: self.1 as i16,
        }
    }
}

/// An object that can be exported as a GDS element.
trait ExportGds {
    /// The GDS type that this object corresponds to.
    type Output;

    /// Exports `self` as its GDS counterpart, accessing and mutating state in `exporter` as needed.
    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output>;
}

impl ExportGds for RawCell {
    type Output = gds::GdsStruct;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let name = exporter.assign_name(self);
        let name_str: &str = self.name.as_ref();

        let span = span!(Level::INFO, "cell", name = name_str);
        let _guard = span.enter();

        let mut cell = gds::GdsStruct::new(name);

        cell.elems.extend(self.ports.export(exporter)?);

        for element in self.elements.iter() {
            if let Some(elem) = element.export(exporter)? {
                cell.elems.push(elem);
            }
        }

        exporter.gds.structs.push(cell.clone());

        Ok(cell)
    }
}

impl ExportGds for HashMap<NameBuf, PortGeometry> {
    type Output = Vec<gds::GdsElement>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let mut elements = Vec::new();
        for (name_buf, geometry) in self {
            elements.extend((name_buf, &geometry.primary).export(exporter)?);
            for shape in geometry.unnamed_shapes.iter() {
                elements.extend((name_buf, shape).export(exporter)?);
            }
            for (_, shape) in geometry.named_shapes.iter() {
                elements.extend((name_buf, shape).export(exporter)?);
            }
        }
        Ok(elements)
    }
}

/// A trait that describes where to place a label for a given shape.
trait PlaceLabels {
    /// Computes a [`Point`] that lies within `self`.
    ///
    /// Allows for placing labels on an arbitrary shape.
    fn label_loc(&self) -> Point;
}

impl PlaceLabels for Shape {
    fn label_loc(&self) -> Point {
        self.shape().label_loc()
    }
}

impl PlaceLabels for geometry::shape::Shape {
    fn label_loc(&self) -> Point {
        match self {
            geometry::shape::Shape::Rect(ref r) => r.label_loc(),
        }
    }
}

impl PlaceLabels for Rect {
    fn label_loc(&self) -> Point {
        self.center()
    }
}

impl ExportGds for (&NameBuf, &IoShape) {
    type Output = Vec<gds::GdsElement>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let (name_buf, shape) = *self;
        let mut elements = Vec::new();
        if let Some(element) =
            Shape::new(shape.layer().pin(), shape.shape().clone()).export(exporter)?
        {
            elements.push(element);
        }
        if let Some(element) = Text::new(
            shape.layer().label(),
            name_buf.to_string(),
            shape.shape().label_loc(),
            NamedOrientation::R0.into_orientation(),
        )
        .export(exporter)?
        {
            elements.push(element.into());
        }
        Ok(elements)
    }
}

impl ExportGds for Element {
    type Output = Option<gds::GdsElement>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "element", element = ?self);
        let _guard = span.enter();

        Ok(match self {
            Element::Instance(instance) => Some(instance.export(exporter)?.into()),
            Element::Shape(shape) => shape.export(exporter)?,
            Element::Text(text) => text.export(exporter)?.map(|text| text.into()),
        })
    }
}

impl ExportGds for RawInstance {
    type Output = gds::GdsStructRef;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "instance", instance = ?self);
        let _guard = span.enter();

        let cell_name = if let Some(name) = exporter.get_name(&self.cell) {
            name
        } else {
            self.cell.export(exporter)?.name
        };

        Ok(gds::GdsStructRef {
            name: cell_name,
            xy: self.loc.export(exporter)?,
            strans: Some(self.orientation.export(exporter)?),
            ..Default::default()
        })
    }
}

impl ExportGds for Shape {
    type Output = Option<gds::GdsElement>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "shape", shape = ?self);
        let _guard = span.enter();

        Ok(if let Some(layer) = self.layer().export(exporter)? {
            Some(match self.shape() {
                geometry::shape::Shape::Rect(r) => gds::GdsBoundary {
                    layer: layer.layer,
                    datatype: layer.xtype,
                    xy: r.export(exporter)?,
                    ..Default::default()
                }
                .into(),
            })
        } else {
            None
        })
    }
}

impl ExportGds for Text {
    type Output = Option<gds::GdsTextElem>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "text", text = ?self);
        let _guard = span.enter();

        Ok(if let Some(layer) = self.layer().export(exporter)? {
            Some(gds::GdsTextElem {
                string: self.text().clone(),
                layer: layer.layer,
                texttype: layer.xtype,
                xy: self.loc().export(exporter)?,
                strans: Some(self.orientation().export(exporter)?),
                ..Default::default()
            })
        } else {
            None
        })
    }
}

impl ExportGds for Rect {
    type Output = Vec<gds::GdsPoint>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "rect", rect = ?self);
        let _guard = span.enter();

        let bl = self.corner(Corner::LowerLeft).export(exporter)?;
        let br = self.corner(Corner::LowerRight).export(exporter)?;
        let ur = self.corner(Corner::UpperRight).export(exporter)?;
        let ul = self.corner(Corner::UpperLeft).export(exporter)?;
        Ok(vec![bl.clone(), br, ur, ul, bl])
    }
}

impl ExportGds for Orientation {
    type Output = gds::GdsStrans;

    fn export(&self, _exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "orientation", orientation = ?self);
        let _guard = span.enter();

        Ok(gds::GdsStrans {
            reflected: self.reflect_vert(),
            angle: Some(self.angle()),
            ..Default::default()
        })
    }
}

impl ExportGds for Point {
    type Output = gds::GdsPoint;

    fn export(&self, _exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "point", point = ?self);
        let _guard = span.enter();

        let x = self.x.try_into().map_err(|e| {
            tracing::event!(
                Level::ERROR,
                "failed to convert coordinate to i32: {}",
                self.x
            );
            e
        })?;
        let y = self.y.try_into().map_err(|e| {
            tracing::event!(
                Level::ERROR,
                "failed to convert coordinate to i32: {}",
                self.x
            );
            e
        })?;
        Ok(gds::GdsPoint::new(x, y))
    }
}

impl ExportGds for LayerId {
    type Output = Option<gds::GdsLayerSpec>;

    fn export(&self, exporter: &mut GdsExporter<'_>) -> GdsExportResult<Self::Output> {
        let span = span!(Level::INFO, "layer ID", layer_id = ?self);
        let _guard = span.enter();

        let spec = exporter.get_layer(*self).map(|spec| spec.into());

        if spec.is_none() {
            tracing::event!(
                Level::WARN,
                "skipping export of layer {:?} as no corresponding GDS layer was found",
                self
            );
        }

        Ok(spec)
    }
}
