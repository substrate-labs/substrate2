//! Utilities for writing netlisters for SCIR libraries.

use crate::schema::Schema;
use crate::{BinOp, Cell, CellId, ChildId, Expr, InstanceId, Library, SignalInfo, Slice};
use arcstr::ArcStr;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{Result, Write};
use std::path::PathBuf;

/// A netlist include statement.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Include {
    /// The path to include.
    pub path: PathBuf,
    /// The section of the provided file to include.
    pub section: Option<ArcStr>,
}

impl<T: Into<PathBuf>> From<T> for Include {
    fn from(value: T) -> Self {
        Self {
            path: value.into(),
            section: None,
        }
    }
}

impl Include {
    /// Creates a new [`Include`].
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self::from(path)
    }

    /// Returns a new [`Include`] with the given section.
    pub fn section(mut self, section: impl Into<ArcStr>) -> Self {
        self.section = Some(section.into());
        self
    }
}

/// Metadata associated with the conversion from a SCIR library to a netlist.
#[derive(Debug, Clone, Default)]
pub struct NetlistLibConversion {
    /// Conversion metadata for each cell in the SCIR library.
    pub cells: HashMap<CellId, NetlistCellConversion>,
}

impl NetlistLibConversion {
    /// Creates a new [`NetlistLibConversion`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// Metadata associated with the conversion from a SCIR cell to a netlisted subcircuit.
#[derive(Debug, Clone, Default)]
pub struct NetlistCellConversion {
    /// The netlisted names of SCIR instances.
    pub instances: HashMap<InstanceId, ArcStr>,
}

impl NetlistCellConversion {
    /// Creates a new [`NetlistCellConversion`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// A schema with a SPICE-like netlist format.
///
/// Appropriate newlines will be added after each function call, so newlines added by
/// implementors may cause formatting issues.
pub trait HasSpiceLikeNetlist: Schema {
    /// Writes a prelude to the beginning of the output stream.
    #[allow(unused_variables)]
    fn write_prelude<W: Write>(&self, out: &mut W, lib: &Library<Self>) -> Result<()> {
        Ok(())
    }
    /// Writes an include statement.
    fn write_include<W: Write>(&self, out: &mut W, include: &Include) -> Result<()>;
    /// Writes a begin subcircuit statement.
    fn write_start_subckt<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        ports: &[&SignalInfo],
    ) -> Result<()>;
    /// Writes an end subcircuit statement.
    fn write_end_subckt<W: Write>(&self, out: &mut W, name: &ArcStr) -> Result<()>;
    /// Writes a SCIR instance.
    fn write_instance<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        connections: impl Iterator<Item = ArcStr>,
        child: &ArcStr,
    ) -> Result<ArcStr>;
    /// Writes a primitive subcircuit.
    ///
    /// Should include a newline after if needed.
    fn write_primitive_subckt<W: Write>(
        &self,
        out: &mut W,
        primitive: &<Self as Schema>::Primitive,
    ) -> Result<()> {
        Ok(())
    }
    /// Writes a primitive instantiation.
    fn write_primitive_inst<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        connections: HashMap<ArcStr, impl Iterator<Item = ArcStr>>,
        primitive: &<Self as Schema>::Primitive,
    ) -> Result<ArcStr>;
    /// Writes the parameters of a primitive device immediately following the written ending.
    fn write_params<W: Write>(&self, out: &mut W, params: &HashMap<ArcStr, Expr>) -> Result<()> {
        for (key, value) in params.iter().sorted_by_key(|(key, _)| *key) {
            write!(out, " {key}=")?;
            self.write_expr(out, value)?;
        }
        Ok(())
    }
    /// Writes a slice.
    fn write_slice<W: Write>(&self, out: &mut W, slice: Slice, info: &SignalInfo) -> Result<()> {
        if let Some(range) = slice.range() {
            for i in range.indices() {
                if i > range.start() {
                    write!(out, " ")?;
                }
                write!(out, "{}[{}]", &info.name, i)?;
            }
        } else {
            write!(out, "{}", &info.name)?;
        }
        Ok(())
    }
    /// Writes a SCIR expression.
    fn write_expr<W: Write>(&self, out: &mut W, expr: &Expr) -> Result<()> {
        match expr {
            Expr::NumericLiteral(dec) => write!(out, "{}", dec)?,
            // boolean literals have no spectre value
            Expr::BoolLiteral(_) => (),
            Expr::StringLiteral(s) | Expr::Var(s) => write!(out, "{}", s)?,
            Expr::BinOp { op, left, right } => {
                write!(out, "(")?;
                self.write_expr(out, left)?;
                write!(out, ")")?;
                match op {
                    BinOp::Add => write!(out, "+")?,
                    BinOp::Sub => write!(out, "-")?,
                    BinOp::Mul => write!(out, "*")?,
                    BinOp::Div => write!(out, "/")?,
                };
                write!(out, "(")?;
                self.write_expr(out, right)?;
                write!(out, ")")?;
                todo!();
            }
        }
        Ok(())
    }
    /// Writes a postlude to the end of the output stream.
    #[allow(unused_variables)]
    fn write_postlude<W: Write>(&self, out: &mut W, lib: &Library<Self>) -> Result<()> {
        Ok(())
    }
}

/// An enumeration describing whether the ground node of a testbench should be renamed.
#[derive(Clone, Debug)]
pub enum RenameGround {
    /// The ground node should be renamed to the provided [`ArcStr`].
    Yes(ArcStr),
    /// The ground node should not be renamed.
    No,
}

/// The type of netlist to be exported.
#[derive(Clone, Debug)]
#[enumify::enumify(no_as_ref, no_as_mut)]
pub enum NetlistKind {
    /// A testbench netlist that should have its top cell inlined and its ground renamed to
    /// the simulator ground node.
    Testbench(RenameGround),
    /// A netlist that is a collection of cells.
    Cells,
}

/// An instance of a netlister.
pub struct NetlisterInstance<'a, S: Schema, W> {
    kind: NetlistKind,
    schema: &'a S,
    lib: &'a Library<S>,
    includes: &'a [Include],
    out: &'a mut W,
}

impl<'a, S: Schema, W> NetlisterInstance<'a, S, W> {
    /// Creates a new [`NetlisterInstance`].
    pub fn new(
        kind: NetlistKind,
        schema: &'a S,
        lib: &'a Library<S>,
        includes: &'a [Include],
        out: &'a mut W,
    ) -> Self {
        Self {
            kind,
            schema,
            lib,
            includes,
            out,
        }
    }
}

impl<'a, S: HasSpiceLikeNetlist, W: Write> NetlisterInstance<'a, S, W> {
    /// Exports a SCIR library to the output stream using a [`SpiceLikeNetlister`].
    pub fn export(mut self) -> Result<NetlistLibConversion> {
        let lib = self.export_library()?;
        self.out.flush()?;
        Ok(lib)
    }

    fn export_library(&mut self) -> Result<NetlistLibConversion> {
        self.schema.write_prelude(self.out, self.lib)?;
        for include in self.includes {
            self.schema.write_include(self.out, include)?;
            writeln!(self.out)?;
        }
        writeln!(self.out)?;

        let mut conv = NetlistLibConversion::new();

        for (id, cell) in self.lib.cells() {
            conv.cells
                .insert(id, self.export_cell(cell, self.lib.is_top(id))?);
        }

        for (id, prim) in self.lib.primitives() {
            self.schema.write_primitive_subckt(self.out, prim)?;
        }

        self.schema.write_postlude(self.out, self.lib)?;
        Ok(conv)
    }

    fn export_cell(&mut self, cell: &Cell, is_top: bool) -> Result<NetlistCellConversion> {
        let is_testbench_top = is_top && self.kind.is_testbench();

        let indent = if is_testbench_top { "" } else { "  " };

        let ground = match (is_testbench_top, &self.kind) {
            (true, NetlistKind::Testbench(RenameGround::Yes(replace_with))) => {
                let msg = "testbench should have one port: ground";
                let mut ports = cell.ports();
                let ground = ports.next().expect(msg);
                assert!(ports.next().is_none(), "{}", msg);
                let ground = &cell.signal(ground.signal()).name;
                Some((ground.clone(), replace_with.clone()))
            }
            _ => None,
        };

        if !is_testbench_top {
            let ports: Vec<&SignalInfo> = cell
                .ports()
                .map(|port| cell.signal(port.signal()))
                .collect();
            self.schema
                .write_start_subckt(self.out, cell.name(), &ports)?;
            writeln!(self.out, "\n")?;
        }

        let mut conv = NetlistCellConversion::new();
        for (id, inst) in cell.instances.iter() {
            write!(self.out, "{}", indent)?;
            let mut connections: HashMap<_, _> = inst
                .connections()
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        v.parts()
                            .map(|part| self.make_slice(cell, *part, &ground))
                            .collect::<Result<Vec<_>>>()?
                            .into_iter(),
                    ))
                })
                .collect::<Result<_>>()?;
            let name = match inst.child() {
                ChildId::Cell(child_id) => {
                    let child = self.lib.cell(child_id);
                    let ports = child.ports().flat_map(|port| {
                        let port_name = &child.signal(port.signal()).name;
                        connections.remove(port_name).unwrap()
                    });
                    self.schema
                        .write_instance(self.out, inst.name(), ports, child.name())?
                }
                ChildId::Primitive(child_id) => {
                    let child = self.lib.primitive(child_id);
                    self.schema
                        .write_primitive_inst(self.out, inst.name(), connections, child)?
                }
            };
            conv.instances.insert(*id, name);
            self.schema.write_params(self.out, inst.params())?;
            writeln!(self.out)?;
        }

        if !is_testbench_top {
            writeln!(self.out)?;
            self.schema.write_end_subckt(self.out, cell.name())?;
            writeln!(self.out, "\n")?;
        }
        Ok(conv)
    }

    fn make_slice(
        &mut self,
        cell: &Cell,
        slice: Slice,
        rename_ground: &Option<(ArcStr, ArcStr)>,
    ) -> Result<ArcStr> {
        let sig_info = cell.signal(slice.signal());
        if let Some((signal, replace_with)) = rename_ground {
            if signal == &sig_info.name && slice.range().is_none() {
                // Ground renaming cannot apply to buses.
                // TODO assert that the ground port has width 1.
                return Ok(arcstr::format!("{}", replace_with));
            }
        }
        let mut buf = Vec::new();
        self.schema.write_slice(&mut buf, slice, sig_info)?;
        Ok(ArcStr::from(std::str::from_utf8(&buf).expect(
            "slice should only have UTF8-compatible characters",
        )))
    }
}
