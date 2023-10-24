//! ngspice plugin for Substrate.
#![warn(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::io::Write;
#[cfg(any(unix, target_os = "redox"))]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;

use crate::blocks::Vsource;
use crate::tran::{Tran, TranCurrentKey, TranOutput, TranVoltageKey};
use arcstr::ArcStr;
use cache::error::TryInnerError;
use cache::CacheableWithState;
use error::*;
use nutlex::parser::Data;
use scir::netlist::{
    HasSpiceLikeNetlist, Include, NetlistKind, NetlistLibConversion, NetlisterInstance,
    RenameGround,
};
use scir::schema::{FromSchema, NoSchema, NoSchemaError};
use scir::{Library, SignalInfo, SliceOnePath};
use serde::{Deserialize, Serialize};
use substrate::block::Block;
use substrate::execute::Executor;
use substrate::io::SchematicType;
use substrate::schematic::primitives::{RawInstance, Resistor};
use substrate::schematic::schema::Schema;
use substrate::schematic::{Primitive, PrimitiveSchematic};
use substrate::simulation::{SimulationContext, Simulator};
use substrate::spice::{self, Spice};
use templates::{write_run_script, RunScriptContext};

pub mod blocks;
pub mod error;
pub(crate) mod templates;
pub mod tran;

/// ngspice primitives.
#[derive(Debug, Clone)]
pub enum NgspicePrimitive {
    /// A SPICE primitive.
    Spice(spice::Primitive),
    /// A voltage source with ports "P" and "N".
    Vsource(Vsource),
}

/// Contents of a ngspice save statement.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SaveStmt {
    /// A raw string to follow ".save".
    Raw(ArcStr),
    /// A SCIR signal path representing a node whose voltage should be saved.
    ScirVoltage(SliceOnePath),
    /// A SCIR signal path representing a resistor whose current should be saved.
    ResistorCurrent(scir::InstancePath),
}

impl<T: Into<ArcStr>> From<T> for SaveStmt {
    fn from(value: T) -> Self {
        Self::Raw(value.into())
    }
}

impl SaveStmt {
    /// Creates a new [`SaveStmt`].
    pub fn new(path: impl Into<ArcStr>) -> Self {
        Self::from(path)
    }

    pub(crate) fn to_save_string(
        &self,
        lib: &Library<Ngspice>,
        conv: &NetlistLibConversion,
    ) -> ArcStr {
        match self {
            SaveStmt::Raw(raw) => raw.clone(),
            SaveStmt::ScirVoltage(scir) => arcstr::format!(
                "v({})",
                node_voltage_path(lib, conv, &lib.simplify_path(scir.clone()),)
            ),
            SaveStmt::ResistorCurrent(scir) => {
                arcstr::format!(
                    "@{}{}[i]",
                    if scir.len() == 1 { "" } else { "R." },
                    instance_path(lib, conv, scir)
                )
            }
        }
    }

    pub(crate) fn to_data_string(
        &self,
        lib: &Library<Ngspice>,
        conv: &NetlistLibConversion,
    ) -> ArcStr {
        match self {
            SaveStmt::Raw(raw) => raw.clone(),
            SaveStmt::ScirVoltage(_) => self.to_save_string(lib, conv),
            SaveStmt::ResistorCurrent(_) => {
                arcstr::format!("i({})", self.to_save_string(lib, conv).to_lowercase())
            }
        }
    }
}

/// Contents of a ngspice probe statement.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ProbeStmt {
    /// A raw string to follow ".probe".
    Raw(ArcStr),
    /// A SCIR signal path representing a terminal whose current should be saved.
    ScirCurrent(SliceOnePath),
}

impl<T: Into<ArcStr>> From<T> for ProbeStmt {
    fn from(value: T) -> Self {
        Self::Raw(value.into())
    }
}

impl ProbeStmt {
    /// Creates a new [`ProbeStmt`].
    pub fn new(path: impl Into<ArcStr>) -> Self {
        Self::from(path)
    }

    pub(crate) fn to_probe_string(
        &self,
        lib: &Library<Ngspice>,
        conv: &NetlistLibConversion,
    ) -> ArcStr {
        match self {
            ProbeStmt::Raw(raw) => raw.clone(),
            ProbeStmt::ScirCurrent(scir) => {
                arcstr::format!("i({})", node_current_path(lib, conv, scir, true))
            }
        }
    }

    pub(crate) fn to_data_string(
        &self,
        lib: &Library<Ngspice>,
        conv: &NetlistLibConversion,
    ) -> ArcStr {
        match self {
            ProbeStmt::Raw(raw) => raw.clone(),
            ProbeStmt::ScirCurrent(scir) => {
                arcstr::format!(
                    "i({})",
                    node_current_path(lib, conv, scir, false).to_lowercase()
                )
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum SavedData {
    Save(SaveStmt),
    Probe(ProbeStmt),
}

impl SavedData {
    pub(crate) fn netlist<W: Write>(
        &self,
        out: &mut W,
        lib: &Library<Ngspice>,
        conv: &NetlistLibConversion,
    ) -> std::io::Result<()> {
        match self {
            Self::Save(save) => write!(out, ".save {}", save.to_save_string(lib, conv)),
            Self::Probe(probe) => write!(out, ".probe {}", probe.to_probe_string(lib, conv)),
        }
    }

    pub(crate) fn to_data_string(
        &self,
        lib: &Library<Ngspice>,
        conv: &NetlistLibConversion,
    ) -> ArcStr {
        match self {
            Self::Save(save) => save.to_data_string(lib, conv),
            Self::Probe(probe) => probe.to_data_string(lib, conv),
        }
    }
}

impl From<SaveStmt> for SavedData {
    fn from(value: SaveStmt) -> Self {
        Self::Save(value)
    }
}

impl From<ProbeStmt> for SavedData {
    fn from(value: ProbeStmt) -> Self {
        Self::Probe(value)
    }
}

/// ngspice simulator global configuration.
#[derive(Debug, Clone, Default)]
pub struct Ngspice {}

/// ngspice per-simulation options.
///
/// A single simulation contains zero or more analyses.
#[derive(Debug, Clone, Default)]
pub struct Options {
    includes: HashSet<Include>,
    saves: HashMap<SavedData, u64>,
    next_save_key: u64,
}

impl Options {
    /// Include the given file.
    pub fn include(&mut self, path: impl Into<PathBuf>) {
        self.includes.insert(Include::new(path));
    }
    /// Include the given section of a file.
    pub fn include_section(&mut self, path: impl Into<PathBuf>, section: impl Into<ArcStr>) {
        self.includes.insert(Include::new(path).section(section));
    }

    fn save_inner(&mut self, save: impl Into<SavedData>) -> u64 {
        let save = save.into();

        if let Some(key) = self.saves.get(&save) {
            *key
        } else {
            let save_key = self.next_save_key;
            self.next_save_key += 1;
            self.saves.insert(save, save_key);
            save_key
        }
    }

    /// Marks a transient voltage to be saved in all transient analyses.
    pub fn save_tran_voltage(&mut self, save: impl Into<SaveStmt>) -> TranVoltageKey {
        TranVoltageKey(self.save_inner(save.into()))
    }

    /// Marks a transient current to be saved in all transient analyses.
    pub fn save_tran_current(&mut self, save: impl Into<SaveStmt>) -> TranCurrentKey {
        TranCurrentKey(vec![self.save_inner(save.into())])
    }

    /// Marks a transient current to be saved in all transient analyses.
    pub fn probe_tran_current(&mut self, save: impl Into<ProbeStmt>) -> TranCurrentKey {
        TranCurrentKey(vec![self.save_inner(save.into())])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
struct CachedSim {
    simulation_netlist: Vec<u8>,
}

struct CachedSimState {
    input: Vec<Input>,
    netlist: PathBuf,
    output_file: PathBuf,
    log: PathBuf,
    err_log: PathBuf,
    run_script: PathBuf,
    work_dir: PathBuf,
    executor: Arc<dyn Executor>,
}

impl CacheableWithState<CachedSimState> for CachedSim {
    type Output = Vec<HashMap<String, Vec<f64>>>;
    type Error = Arc<Error>;

    fn generate_with_state(
        &self,
        state: CachedSimState,
    ) -> std::result::Result<Self::Output, Self::Error> {
        let inner = || -> Result<Self::Output> {
            let CachedSimState {
                input,
                netlist,
                output_file,
                log,
                err_log,
                run_script,
                work_dir,
                executor,
            } = state;
            write_run_script(
                RunScriptContext {
                    netlist: &netlist,
                    raw_output_file: &output_file,
                    log_path: &log,
                    err_path: &err_log,
                    bashrc: None,
                    flags: "",
                },
                &run_script,
            )?;

            let mut perms = std::fs::metadata(&run_script)?.permissions();
            #[cfg(any(unix, target_os = "redox"))]
            perms.set_mode(0o744);
            std::fs::set_permissions(&run_script, perms)?;

            let mut command = std::process::Command::new("/bin/bash");
            command.arg(&run_script).current_dir(&work_dir);
            executor
                .execute(command, Default::default())
                .map_err(|_| Error::NgspiceError)?;

            let contents = std::fs::read(&output_file)?;
            let rawfile = nutlex::parse(&contents)?;

            let mut raw_outputs = Vec::with_capacity(input.len());

            for (an, results) in input.iter().zip(rawfile.analyses.into_iter()) {
                match an {
                    Input::Tran(_) => match results.data {
                        Data::Real(real) => raw_outputs.push(HashMap::from_iter(
                            results
                                .variables
                                .into_iter()
                                .map(|var| (var.name.to_string(), real[var.idx].clone())),
                        )),
                        _ => {
                            return Err(Error::NgspiceError);
                        }
                    },
                }
            }

            Ok(raw_outputs)
        };
        inner().map_err(Arc::new)
    }
}

impl Ngspice {
    fn simulate(
        &self,
        ctx: &SimulationContext<Ngspice>,
        options: Options,
        input: Vec<Input>,
    ) -> Result<Vec<Output>> {
        std::fs::create_dir_all(&ctx.work_dir)?;
        let netlist = ctx.work_dir.join("netlist.spice");
        let mut f = std::fs::File::create(&netlist)?;
        let mut w = Vec::new();

        let mut includes = options.includes.into_iter().collect::<Vec<_>>();
        let mut saves = options.saves.keys().cloned().collect::<Vec<_>>();
        // Sorting the include list makes repeated netlist invocations
        // produce the same output. If we were to iterate over the HashSet directly,
        // the order of includes may change even if the contents of the set did not change.
        includes.sort();
        saves.sort();

        let netlister = NetlisterInstance::new(
            NetlistKind::Testbench(RenameGround::Yes(arcstr::literal!("0"))),
            self,
            &ctx.lib.scir,
            &includes,
            &mut w,
        );
        let conv = netlister.export()?;

        writeln!(w)?;
        for save in saves {
            save.netlist(&mut w, &ctx.lib.scir, &conv)?;
            writeln!(w)?;
        }

        writeln!(w)?;
        for an in input.iter() {
            an.netlist(&mut w)?;
            writeln!(w)?;
        }
        f.write_all(&w)?;

        let output_file = ctx.work_dir.join("data.raw");
        let log = ctx.work_dir.join("ngspice.log");
        let err_log = ctx.work_dir.join("ngspice.err");
        let run_script = ctx.work_dir.join("simulate.sh");
        let work_dir = ctx.work_dir.clone();
        let executor = ctx.executor.clone();

        let raw_outputs = ctx
            .cache
            .get_with_state(
                "ngspice.simulation.outputs",
                CachedSim {
                    simulation_netlist: w,
                },
                CachedSimState {
                    input,
                    netlist,
                    output_file,
                    log,
                    err_log,
                    run_script,
                    work_dir,
                    executor,
                },
            )
            .try_inner()
            .map_err(|e| match e {
                TryInnerError::CacheError(e) => Error::Caching(e),
                TryInnerError::GeneratorError(e) => Error::Generator(e.clone()),
            })?
            .clone();

        let conv = Arc::new(conv);
        let outputs = raw_outputs
            .into_iter()
            .map(|mut raw_values| {
                TranOutput {
                    lib: ctx.lib.clone(),
                    conv: conv.clone(),
                    time: Arc::new(raw_values.remove("time").unwrap()),
                    raw_values: raw_values
                        .into_iter()
                        .map(|(k, v)| (ArcStr::from(k), Arc::new(v)))
                        .collect(),
                    saved_values: options
                        .saves
                        .iter()
                        .map(|(k, v)| (*v, k.to_data_string(&ctx.lib.scir, &conv)))
                        .collect(),
                }
                .into()
            })
            .collect();

        Ok(outputs)
    }
}

impl scir::schema::Schema for Ngspice {
    type Primitive = NgspicePrimitive;
}

impl FromSchema<NoSchema> for Ngspice {
    type Error = NoSchemaError;

    fn convert_primitive(
        _primitive: <NoSchema as Schema>::Primitive,
    ) -> std::result::Result<<Self as Schema>::Primitive, Self::Error> {
        Err(NoSchemaError)
    }

    fn convert_instance(
        _instance: &mut scir::Instance,
        _primitive: &<NoSchema as Schema>::Primitive,
    ) -> std::result::Result<(), Self::Error> {
        Err(NoSchemaError)
    }
}

impl PrimitiveSchematic<Ngspice> for RawInstance {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
    ) -> substrate::schematic::Primitive<Ngspice> {
        let mut prim = Primitive::new(NgspicePrimitive::Spice(spice::Primitive {
            kind: spice::PrimitiveKind::RawInstance {
                cell: self.cell.clone(),
                ports: self.ports.clone(),
            },
            params: self.params.clone(),
        }));
        for (i, port) in self.ports.iter().enumerate() {
            prim.connect(port, io[i]);
        }
        prim
    }
}

impl PrimitiveSchematic<Ngspice> for Resistor {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
    ) -> substrate::schematic::Primitive<Ngspice> {
        let mut prim = Primitive::new(NgspicePrimitive::Spice(spice::Primitive {
            kind: spice::PrimitiveKind::Res2 {
                value: self.value(),
            },
            params: HashMap::new(),
        }));
        prim.connect("1", io.p);
        prim.connect("2", io.n);
        prim
    }
}

impl Simulator for Ngspice {
    type Schema = Ngspice;
    type Input = Input;
    type Options = Options;
    type Output = Output;
    type Error = Error;

    fn simulate_inputs(
        &self,
        config: &substrate::simulation::SimulationContext<Self>,
        options: Self::Options,
        input: Vec<Self::Input>,
    ) -> Result<Vec<Self::Output>> {
        self.simulate(config, options, input)
    }
}

pub(crate) fn instance_path(
    lib: &Library<Ngspice>,
    conv: &NetlistLibConversion,
    path: &scir::InstancePath,
) -> String {
    lib.convert_instance_path_with_conv(conv, path.clone())
        .join(".")
}

pub(crate) fn node_voltage_path(
    lib: &Library<Ngspice>,
    conv: &NetlistLibConversion,
    path: &SliceOnePath,
) -> String {
    lib.convert_slice_one_path_with_conv(conv, path.clone(), |name, index| {
        if let Some(index) = index {
            arcstr::format!("{}[{}]", name, index)
        } else {
            name.clone()
        }
    })
    .join(".")
}

pub(crate) fn node_current_path(
    lib: &Library<Ngspice>,
    conv: &NetlistLibConversion,
    path: &SliceOnePath,
    _save: bool,
) -> String {
    let mut named_path = lib.convert_slice_one_path_with_conv(conv, path.clone(), |name, index| {
        if let Some(index) = index {
            arcstr::format!("{}\\[{}\\]", name, index)
        } else {
            name.clone()
        }
    });
    let signal = named_path.pop().unwrap();
    let mut str_path = named_path.join(".");
    str_path.push(':');
    str_path.push_str(&signal);
    str_path
}

/// Inputs directly supported by ngspice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Input {
    /// Transient simulation input.
    Tran(Tran),
}

impl From<Tran> for Input {
    fn from(value: Tran) -> Self {
        Self::Tran(value)
    }
}

/// Outputs directly produced by ngspice.
#[derive(Debug, Clone)]
pub enum Output {
    /// Transient simulation output.
    Tran(TranOutput),
}

impl From<TranOutput> for Output {
    fn from(value: TranOutput) -> Self {
        Self::Tran(value)
    }
}

impl TryFrom<Output> for TranOutput {
    type Error = Error;
    fn try_from(value: Output) -> Result<Self> {
        match value {
            Output::Tran(t) => Ok(t),
        }
    }
}

impl Input {
    fn netlist<W: Write>(&self, out: &mut W) -> Result<()> {
        match self {
            Self::Tran(t) => t.netlist(out),
        }
    }
}

impl Tran {
    fn netlist<W: Write>(&self, out: &mut W) -> Result<()> {
        write!(out, ".tran {} {}", self.step, self.stop)?;
        if let Some(ref start) = self.start {
            write!(out, "{start}")?;
        }
        Ok(())
    }
}

impl HasSpiceLikeNetlist for Ngspice {
    fn write_prelude<W: Write>(&self, out: &mut W, lib: &Library<Self>) -> std::io::Result<()> {
        writeln!(out, "* {}", lib.name())?;
        writeln!(out, "* This is a generated file. Be careful when editing manually: this file may be overwritten.\n")?;
        Ok(())
    }

    fn write_include<W: Write>(
        &self,
        out: &mut W,
        include: &scir::netlist::Include,
    ) -> std::io::Result<()> {
        Spice.write_include(out, include)
    }

    fn write_start_subckt<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        ports: &[&SignalInfo],
    ) -> std::io::Result<()> {
        Spice.write_start_subckt(out, name, ports)
    }

    fn write_end_subckt<W: Write>(&self, out: &mut W, name: &ArcStr) -> std::io::Result<()> {
        Spice.write_end_subckt(out, name)
    }

    fn write_instance<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        connections: impl Iterator<Item = ArcStr>,
        child: &ArcStr,
    ) -> std::io::Result<ArcStr> {
        Spice.write_instance(out, name, connections, child)
    }

    fn write_primitive_subckt<W: Write>(
        &self,
        out: &mut W,
        primitive: &<Self as Schema>::Primitive,
    ) -> std::io::Result<()> {
        if let NgspicePrimitive::Spice(spice_primitive) = primitive {
            Spice.write_primitive_subckt(out, spice_primitive)?;
        }
        Ok(())
    }

    fn write_primitive_inst<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        mut connections: HashMap<ArcStr, impl Iterator<Item = ArcStr>>,
        primitive: &<Self as Schema>::Primitive,
    ) -> std::io::Result<ArcStr> {
        match primitive {
            NgspicePrimitive::Spice(spice_primitive) => {
                Spice.write_primitive_inst(out, name, connections, spice_primitive)
            }
            NgspicePrimitive::Vsource(vsource) => {
                let name = arcstr::format!("V{}", name);
                write!(out, "{}", name)?;
                for port in ["P", "N"] {
                    for part in connections.remove(port).unwrap() {
                        write!(out, " {}", part)?;
                    }
                }
                match vsource {
                    Vsource::Dc(dc) => {
                        write!(out, " DC {}", dc)?;
                    }
                    Vsource::Pulse(pulse) => {
                        write!(
                            out,
                            "PULSE({} {} {} {} {} {} {} {})",
                            pulse.val0,
                            pulse.val1,
                            pulse.delay.unwrap_or_default(),
                            pulse.rise.unwrap_or_default(),
                            pulse.fall.unwrap_or_default(),
                            pulse.width.unwrap_or_default(),
                            pulse.period.unwrap_or_default(),
                            pulse.num_pulses.unwrap_or_default(),
                        )?;
                    }
                }
                Ok(name)
            }
        }
    }
}
