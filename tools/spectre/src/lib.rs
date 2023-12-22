//! Spectre plugin for Substrate.
#![warn(missing_docs)]

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Write;
#[cfg(any(unix, target_os = "redox"))]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::sync::Arc;

use crate::tran::Tran;
use arcstr::ArcStr;
use cache::error::TryInnerError;
use cache::CacheableWithState;
use error::*;
use itertools::Itertools;
use psfparser::analysis::transient::TransientData;
use rust_decimal::Decimal;
use scir::schema::{FromSchema, NoSchema, NoSchemaError};
use scir::{
    Library, NamedSliceOne, NetlistLibConversion, ParamValue, SignalInfo, Slice, SliceOnePath,
};
use serde::{Deserialize, Serialize};
use spice::netlist::{
    HasSpiceLikeNetlist, Include, NetlistKind, NetlistOptions, NetlisterInstance, RenameGround,
};
use spice::{BlackboxContents, BlackboxElement, Spice};
use substrate::block::Block;
use substrate::context::Installation;
use substrate::execute::Executor;
use substrate::io::{NodePath, SchematicType};
use substrate::schematic::conv::ConvertedNodePath;
use substrate::schematic::netlist::ConvertibleNetlister;
use substrate::schematic::primitives::{Capacitor, RawInstance, Resistor};
use substrate::schematic::schema::Schema;
use substrate::schematic::{CellBuilder, PrimitiveBinding, Schematic};
use substrate::simulation::options::ic::InitialCondition;
use substrate::simulation::options::{ic, SimOption};
use substrate::simulation::{SimulationContext, Simulator};
use substrate::type_dispatch::impl_dispatch;
use templates::{write_run_script, RunScriptContext};

pub mod blocks;
pub mod error;
pub(crate) mod templates;
pub mod tran;

/// Spectre primitives.
#[derive(Debug, Clone)]
pub enum Primitive {
    /// A raw instance with an associated cell.
    RawInstance {
        /// The associated cell.
        cell: ArcStr,
        /// The ordered ports of the instance.
        ports: Vec<ArcStr>,
        /// Parameters associated with the instance.
        params: HashMap<ArcStr, ParamValue>,
    },
    /// An instance with blackboxed contents.
    BlackboxInstance {
        /// The contents of the cell.
        contents: BlackboxContents,
    },
    /// A SPICE primitive.
    ///
    /// Integrated using `simulator lang=spice`.
    Spice(spice::Primitive),
}

/// Spectre error presets.
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub enum ErrPreset {
    /// Liberal.
    Liberal,
    /// Moderate.
    #[default]
    Moderate,
    /// Conservative.
    Conservative,
}

impl Display for ErrPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Liberal => write!(f, "liberal"),
            Self::Moderate => write!(f, "moderate"),
            Self::Conservative => write!(f, "conservative"),
        }
    }
}

/// A signal referenced by a save/ic Spectre statement.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SimSignal {
    /// A raw string to follow "save/ic".
    Raw(ArcStr),
    /// A SCIR signal path representing a node whose voltage should be referenced.
    ScirVoltage(SliceOnePath),
    /// A SCIR signal path representing a terminal whose current should be referenced.
    ScirCurrent(SliceOnePath),
}

impl<T: Into<ArcStr>> From<T> for SimSignal {
    fn from(value: T) -> Self {
        Self::Raw(value.into())
    }
}

impl SimSignal {
    /// Creates a new [`SimSignal`].
    pub fn new(path: impl Into<ArcStr>) -> Self {
        Self::from(path)
    }

    pub(crate) fn to_string(&self, lib: &Library<Spectre>, conv: &NetlistLibConversion) -> ArcStr {
        match self {
            SimSignal::Raw(raw) => raw.clone(),
            SimSignal::ScirCurrent(scir) => {
                ArcStr::from(Spectre::node_current_path(lib, conv, scir))
            }
            SimSignal::ScirVoltage(scir) => {
                ArcStr::from(Spectre::node_voltage_path(lib, conv, scir))
            }
        }
    }
}

/// Spectre simulator global configuration.
#[derive(Debug, Clone, Default)]
pub struct Spectre {}

/// Spectre per-simulation options.
///
/// A single simulation contains zero or more analyses.
#[derive(Debug, Clone, Default)]
pub struct Options {
    includes: HashSet<Include>,
    saves: HashMap<SimSignal, u64>,
    ics: HashMap<SimSignal, Decimal>,
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

    fn save_inner(&mut self, save: impl Into<SimSignal>) -> u64 {
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

    fn set_ic_inner(&mut self, key: impl Into<SimSignal>, value: Decimal) {
        self.ics.insert(key.into(), value);
    }

    /// Marks a transient voltage to be saved in all transient analyses.
    pub fn save_tran_voltage(&mut self, save: impl Into<SimSignal>) -> tran::VoltageSavedKey {
        tran::VoltageSavedKey(self.save_inner(save))
    }

    /// Marks a transient current to be saved in all transient analyses.
    pub fn save_tran_current(&mut self, save: impl Into<SimSignal>) -> tran::CurrentSavedKey {
        tran::CurrentSavedKey(vec![self.save_inner(save)])
    }
}

#[impl_dispatch({&str; &String; ArcStr; String; SimSignal})]
impl<K> SimOption<Spectre> for InitialCondition<K, ic::Voltage> {
    fn set_option(
        self,
        opts: &mut <Spectre as Simulator>::Options,
        _ctx: &SimulationContext<Spectre>,
    ) {
        opts.set_ic_inner(self.path, *self.value);
    }
}

impl SimOption<Spectre> for InitialCondition<&SliceOnePath, ic::Voltage> {
    fn set_option(
        self,
        opts: &mut <Spectre as Simulator>::Options,
        _ctx: &SimulationContext<Spectre>,
    ) {
        opts.set_ic_inner(SimSignal::ScirVoltage(self.path.clone()), *self.value);
    }
}

impl SimOption<Spectre> for InitialCondition<&ConvertedNodePath, ic::Voltage> {
    fn set_option(
        self,
        opts: &mut <Spectre as Simulator>::Options,
        _ctx: &SimulationContext<Spectre>,
    ) {
        opts.set_ic_inner(
            SimSignal::ScirVoltage(match self.path {
                ConvertedNodePath::Cell(path) => path.clone(),
                ConvertedNodePath::Primitive {
                    instances, port, ..
                } => SliceOnePath::new(instances.clone(), NamedSliceOne::new(port.clone())),
            }),
            *self.value,
        );
    }
}

impl SimOption<Spectre> for InitialCondition<&NodePath, ic::Voltage> {
    fn set_option(
        self,
        opts: &mut <Spectre as Simulator>::Options,
        ctx: &SimulationContext<Spectre>,
    ) {
        InitialCondition {
            path: ctx.lib.convert_node_path(self.path).unwrap(),
            value: self.value,
        }
        .set_option(opts, ctx)
    }
}

#[impl_dispatch({SliceOnePath; ConvertedNodePath; NodePath})]
impl<T> SimOption<Spectre> for InitialCondition<T, ic::Voltage> {
    fn set_option(
        self,
        opts: &mut <Spectre as Simulator>::Options,
        ctx: &SimulationContext<Spectre>,
    ) {
        InitialCondition {
            path: &self.path,
            value: self.value,
        }
        .set_option(opts, ctx)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
struct CachedSim {
    simulation_netlist: Vec<u8>,
}

struct CachedSimState {
    input: Vec<Input>,
    netlist: PathBuf,
    output_path: PathBuf,
    log: PathBuf,
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
                output_path,
                log,
                run_script,
                work_dir,
                executor,
            } = state;
            write_run_script(
                RunScriptContext {
                    netlist: &netlist,
                    raw_output_path: &output_path,
                    log_path: &log,
                    bashrc: None,
                    format: "psfbin",
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
                .map_err(|_| Error::SpectreError)?;

            let mut raw_outputs = Vec::with_capacity(input.len());

            for (i, input) in input.iter().enumerate() {
                let file_name = match input {
                    Input::Tran(_) => {
                        format!("analysis{i}.tran.tran")
                    }
                };
                let psf_path = output_path.join(file_name);
                let psf = std::fs::read(psf_path)?;
                let ast = psfparser::binary::parse(&psf).map_err(|_| Error::Parse)?;
                match input {
                    Input::Tran(_) => {
                        let values = TransientData::from_binary(ast).signals;
                        raw_outputs.push(values);
                    }
                }
            }
            Ok(raw_outputs)
        };
        inner().map_err(Arc::new)
    }
}

impl ConvertibleNetlister<Spectre> for Spectre {
    type Error = std::io::Error;
    type Options<'a> = NetlistOptions<'a>;

    fn write_scir_netlist<W: Write>(
        &self,
        lib: &Library<Spectre>,
        out: &mut W,
        opts: Self::Options<'_>,
    ) -> std::result::Result<NetlistLibConversion, Self::Error> {
        NetlisterInstance::new(self, lib, out, opts).export()
    }
}

impl Spectre {
    fn simulate(
        &self,
        ctx: &SimulationContext<Self>,
        options: Options,
        input: Vec<Input>,
    ) -> Result<Vec<Output>> {
        std::fs::create_dir_all(&ctx.work_dir)?;
        let netlist = ctx.work_dir.join("netlist.scs");
        let mut f = std::fs::File::create(&netlist)?;
        let mut w = Vec::new();

        let mut includes = options.includes.into_iter().collect::<Vec<_>>();
        let mut saves = options.saves.keys().cloned().collect::<Vec<_>>();
        let mut ics = options
            .ics
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect::<Vec<_>>();
        // Sorting the include list makes repeated netlist invocations
        // produce the same output. If we were to iterate over the HashSet directly,
        // the order of includes may change even if the contents of the set did not change.
        includes.sort();
        saves.sort();
        ics.sort();

        let conv = self.write_scir_netlist(
            &ctx.lib.scir,
            &mut w,
            NetlistOptions::new(
                NetlistKind::Testbench(RenameGround::Yes("0".into())),
                &includes,
            ),
        )?;

        writeln!(w)?;
        for save in saves {
            writeln!(w, "save {}", save.to_string(&ctx.lib.scir, &conv))?;
        }
        for (k, v) in ics {
            writeln!(w, "ic {}={}", k.to_string(&ctx.lib.scir, &conv), v)?;
        }

        writeln!(w)?;
        for (i, an) in input.iter().enumerate() {
            write!(w, "analysis{i} ")?;
            an.netlist(&mut w)?;
            writeln!(w)?;
        }
        f.write_all(&w)?;

        let output_path = ctx.work_dir.join("psf");
        let log = ctx.work_dir.join("spectre.log");
        let run_script = ctx.work_dir.join("simulate.sh");
        let work_dir = ctx.work_dir.clone();
        let executor = ctx.ctx.executor.clone();

        let raw_outputs = ctx
            .ctx
            .cache
            .get_with_state(
                "spectre.simulation.outputs",
                CachedSim {
                    simulation_netlist: w,
                },
                CachedSimState {
                    input,
                    netlist,
                    output_path,
                    log,
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
                tran::Output {
                    time: Arc::new(raw_values.remove("time").unwrap()),
                    raw_values: raw_values
                        .into_iter()
                        .map(|(k, v)| (ArcStr::from(k), Arc::new(v)))
                        .collect(),
                    saved_values: options
                        .saves
                        .iter()
                        .map(|(k, v)| (*v, k.to_string(&ctx.lib.scir, &conv)))
                        .collect(),
                }
                .into()
            })
            .collect();

        Ok(outputs)
    }

    /// Escapes the given identifier to be Spectre-compatible.
    pub fn escape_identifier(node_name: &str) -> String {
        let mut escaped_name = String::new();
        for c in node_name.chars() {
            if c.is_alphanumeric() || c == '_' {
                escaped_name.push(c);
            } else {
                escaped_name.push('\\');
                escaped_name.push(c);
            }
        }
        escaped_name
    }

    /// Converts a [`scir::InstancePath`] to a Spectre path string corresponding to
    /// the associated instance.
    pub fn instance_path(
        lib: &Library<Spectre>,
        conv: &NetlistLibConversion,
        path: &scir::InstancePath,
    ) -> String {
        lib.convert_instance_path_with_conv(conv, path.clone())
            .join(".")
    }

    /// Converts a [`SliceOnePath`] to a Spectre path string corresponding to the associated
    /// node voltage.
    pub fn node_voltage_path(
        lib: &Library<Spectre>,
        conv: &NetlistLibConversion,
        path: &SliceOnePath,
    ) -> String {
        lib.convert_slice_one_path_with_conv(conv, path.clone(), |name, index| {
            let name = Spectre::escape_identifier(name);
            if let Some(index) = index {
                arcstr::format!("{}\\[{}\\]", name, index)
            } else {
                name.into()
            }
        })
        .join(".")
    }

    /// Converts a [`SliceOnePath`] to a Spectre path string corresponding to the associated
    /// terminal current.
    pub fn node_current_path(
        lib: &Library<Spectre>,
        conv: &NetlistLibConversion,
        path: &SliceOnePath,
    ) -> String {
        let mut named_path =
            lib.convert_slice_one_path_with_conv(conv, path.clone(), |name, index| {
                let name = Spectre::escape_identifier(name);
                if let Some(index) = index {
                    arcstr::format!("{}\\[{}\\]", name, index)
                } else {
                    name.into()
                }
            });
        let signal = named_path.pop().unwrap();
        let mut str_path = named_path.join(".");
        str_path.push(':');
        str_path.push_str(&signal);
        str_path
    }
}

impl scir::schema::Schema for Spectre {
    type Primitive = Primitive;
}

impl FromSchema<NoSchema> for Spectre {
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

/// An error converting to/from the [`Spectre`] schema.
#[derive(Debug, Clone, Copy)]
pub enum SpectreConvError {
    /// A primitive that is not supported by the target schema was encountered.
    UnsupportedPrimitive,
    /// A primitive is missing a required parameter.
    MissingParameter,
    /// A primitive has an extra parameter.
    ExtraParameter,
    /// A primitive has an invalid value for a certain parameter.
    InvalidParameter,
    /// A primitive has an invalid port.
    InvalidPort,
}

impl FromSchema<Spice> for Spectre {
    type Error = SpectreConvError;

    fn convert_primitive(
        primitive: <Spice as Schema>::Primitive,
    ) -> std::result::Result<<Self as Schema>::Primitive, Self::Error> {
        Ok(match primitive {
            spice::Primitive::Res2 { value } => Primitive::RawInstance {
                cell: "resistor".into(),
                ports: vec!["1".into(), "2".into()],
                params: HashMap::from_iter([("r".into(), value.into())]),
            },
            primitive => Primitive::Spice(primitive),
        })
    }

    fn convert_instance(
        _instance: &mut scir::Instance,
        _primitive: &<Spice as Schema>::Primitive,
    ) -> std::result::Result<(), Self::Error> {
        Ok(())
    }
}

impl Schematic<Spectre> for Resistor {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<Spectre>,
    ) -> substrate::error::Result<Self::NestedData> {
        let mut prim = PrimitiveBinding::new(Primitive::RawInstance {
            cell: arcstr::literal!("resistor"),
            ports: vec![arcstr::literal!("1"), arcstr::literal!("2")],
            params: HashMap::from_iter([(
                arcstr::literal!("r"),
                ParamValue::Numeric(self.value()),
            )]),
        });
        prim.connect("1", io.p);
        prim.connect("2", io.n);
        cell.set_primitive(prim);
        Ok(())
    }
}

impl Schematic<Spectre> for Capacitor {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<Spectre>,
    ) -> substrate::error::Result<Self::NestedData> {
        let mut prim = PrimitiveBinding::new(Primitive::RawInstance {
            cell: arcstr::literal!("capacitor"),
            ports: vec![arcstr::literal!("1"), arcstr::literal!("2")],
            params: HashMap::from_iter([(
                arcstr::literal!("c"),
                ParamValue::Numeric(self.value()),
            )]),
        });
        prim.connect("1", io.p);
        prim.connect("2", io.n);
        cell.set_primitive(prim);
        Ok(())
    }
}

impl Schematic<Spectre> for RawInstance {
    fn schematic(
        &self,
        io: &<<Self as Block>::Io as SchematicType>::Bundle,
        cell: &mut CellBuilder<Spectre>,
    ) -> substrate::error::Result<Self::NestedData> {
        let mut prim = PrimitiveBinding::new(Primitive::RawInstance {
            cell: self.cell.clone(),
            ports: self.ports.clone(),
            params: self.params.clone(),
        });
        for (i, port) in self.ports.iter().enumerate() {
            prim.connect(port, io[i]);
        }
        cell.set_primitive(prim);
        Ok(())
    }
}

impl Installation for Spectre {}

impl Simulator for Spectre {
    type Schema = Spectre;
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

/// Inputs directly supported by Spectre.
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

/// Outputs directly produced by Spectre.
#[derive(Debug, Clone)]
pub enum Output {
    /// Transient simulation output.
    Tran(tran::Output),
}

impl From<tran::Output> for Output {
    fn from(value: tran::Output) -> Self {
        Self::Tran(value)
    }
}

impl TryFrom<Output> for tran::Output {
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
        write!(out, "tran stop={}", self.stop)?;
        if let Some(ref start) = self.start {
            write!(out, " start={start}")?;
        }
        if let Some(errpreset) = self.errpreset {
            write!(out, " errpreset={errpreset}")?;
        }
        Ok(())
    }
}

impl HasSpiceLikeNetlist for Spectre {
    fn write_prelude<W: Write>(&self, out: &mut W, _lib: &Library<Spectre>) -> std::io::Result<()> {
        writeln!(out, "// Substrate Spectre library\n")?;
        writeln!(out, "simulator lang=spectre\n")?;
        writeln!(out, "// This is a generated file.")?;
        writeln!(
            out,
            "// Be careful when editing manually: this file may be overwritten.\n"
        )?;
        Ok(())
    }

    fn write_include<W: Write>(&self, out: &mut W, include: &Include) -> std::io::Result<()> {
        if let Some(section) = &include.section {
            write!(out, "include {:?} section={}", include.path, section)?;
        } else {
            write!(out, "include {:?}", include.path)?;
        }
        Ok(())
    }

    fn write_start_subckt<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        ports: &[&SignalInfo],
    ) -> std::io::Result<()> {
        write!(out, "subckt {} (", name)?;
        for sig in ports {
            if let Some(width) = sig.width {
                for i in 0..width {
                    write!(out, " {}\\[{}\\]", sig.name, i)?;
                }
            } else {
                write!(out, " {}", sig.name)?;
            }
        }
        write!(out, " )")?;
        Ok(())
    }

    fn write_end_subckt<W: Write>(&self, out: &mut W, name: &ArcStr) -> std::io::Result<()> {
        write!(out, "ends {}", name)
    }

    fn write_instance<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        connections: Vec<ArcStr>,
        child: &ArcStr,
    ) -> std::io::Result<ArcStr> {
        let name = ArcStr::from(Spectre::escape_identifier(&format!("x{}", name)));
        write!(out, "{} (", name)?;

        for connection in connections {
            write!(out, " {}", connection)?;
        }

        write!(out, " ) {}", child)?;

        Ok(name)
    }

    fn write_primitive_inst<W: Write>(
        &self,
        out: &mut W,
        name: &ArcStr,
        mut connections: HashMap<ArcStr, Vec<ArcStr>>,
        primitive: &<Self as Schema>::Primitive,
    ) -> std::io::Result<ArcStr> {
        Ok(match primitive {
            Primitive::RawInstance {
                cell,
                ports,
                params,
            } => {
                let connections = ports
                    .iter()
                    .flat_map(|port| connections.remove(port).unwrap())
                    .collect();
                let name = self.write_instance(out, name, connections, cell)?;
                for (key, value) in params.iter().sorted_by_key(|(key, _)| *key) {
                    write!(out, " {key}={value}")?;
                }
                name
            }
            Primitive::BlackboxInstance { contents } => {
                // TODO: See if there is a way to translate the name based on the
                // contents, or make documentation explaining that blackbox instances
                // cannot be addressed by path.
                for elem in &contents.elems {
                    match elem {
                        BlackboxElement::InstanceName => write!(out, "{}", name)?,
                        BlackboxElement::RawString(s) => write!(out, "{}", s)?,
                        BlackboxElement::Port(p) => {
                            for part in connections.get(p).unwrap() {
                                write!(out, "{}", part)?
                            }
                        }
                    }
                }
                name.clone()
            }
            Primitive::Spice(p) => {
                writeln!(out, "simulator lang=spice")?;
                let name = Spice.write_primitive_inst(out, name, connections, p)?;
                writeln!(out, "simulator lang=spectre")?;
                name
            }
        })
    }

    fn write_slice<W: Write>(
        &self,
        out: &mut W,
        slice: Slice,
        info: &SignalInfo,
    ) -> std::io::Result<()> {
        let name = Spectre::escape_identifier(&info.name);
        if let Some(range) = slice.range() {
            for i in range.indices() {
                write!(out, "{}\\[{}\\]", &name, i)?;
            }
        } else {
            write!(out, "{}", &name)?;
        }
        Ok(())
    }
}
