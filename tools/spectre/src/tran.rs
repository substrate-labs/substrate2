//! Spectre transient analysis options and data structures.

use crate::{node_voltage_path, ErrPreset, SimSignal, Spectre};
use arcstr::ArcStr;
use rust_decimal::Decimal;
use scir::NetlistLibConversion;
use scir::{NamedSliceOne, SliceOnePath};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use substrate::io::{NodePath, TerminalPath};
use substrate::schematic::conv::{ConvertedNodePath, RawLib};
use substrate::schematic::{Cell, ExportsNestedData};
use substrate::simulation::data::{tran, FromSaved, Save};
use substrate::simulation::{Analysis, SimulationContext, Simulator, SupportedBy};
use substrate::type_dispatch::impl_dispatch;

/// A transient analysis.
#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tran {
    /// Stop time (sec).
    pub stop: Decimal,
    /// Start time (sec).
    ///
    /// Defaults to 0.
    pub start: Option<Decimal>,

    /// The error preset.
    pub errpreset: Option<ErrPreset>,
}

/// The result of a transient analysis.
#[derive(Debug, Clone)]
pub struct Output {
    pub(crate) lib: Arc<RawLib<Spectre>>,
    pub(crate) conv: Arc<NetlistLibConversion>,
    /// The time points of the transient simulation.
    pub time: Arc<Vec<f64>>,
    /// A map from signal name to values.
    pub raw_values: HashMap<ArcStr, Arc<Vec<f64>>>,
    /// A map from a save ID to a raw value identifier.
    pub(crate) saved_values: HashMap<u64, ArcStr>,
}

impl FromSaved<Spectre, Tran> for Output {
    type Key = ();
    fn from_saved(output: &<Tran as Analysis>::Output, _key: Self::Key) -> Self {
        (*output).clone()
    }
}

impl<T: ExportsNestedData> Save<Spectre, Tran, &Cell<T>> for Output {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        _to_save: &Cell<T>,
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

impl Save<Spectre, Tran, ()> for Output {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        _to_save: (),
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

impl FromSaved<Spectre, Tran> for tran::Time {
    type Key = ();
    fn from_saved(output: &<Tran as Analysis>::Output, _key: Self::Key) -> Self {
        tran::Time(output.time.clone())
    }
}

impl<T: ExportsNestedData> Save<Spectre, Tran, &Cell<T>> for tran::Time {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        _to_save: &Cell<T>,
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

impl Save<Spectre, Tran, ()> for tran::Time {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        _to_save: (),
        _opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
    }
}

/// An identifier for a saved transient voltage.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoltageKey(pub(crate) u64);

impl FromSaved<Spectre, Tran> for tran::Voltage {
    type Key = VoltageKey;
    fn from_saved(output: &<Tran as Analysis>::Output, key: Self::Key) -> Self {
        tran::Voltage(
            output
                .raw_values
                .get(output.saved_values.get(&key.0).unwrap())
                .unwrap()
                .clone(),
        )
    }
}

#[impl_dispatch({&str; &String; ArcStr; String; SimSignal})]
impl<T> Save<Spectre, Tran, T> for tran::Voltage {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_voltage(to_save)
    }
}

impl Save<Spectre, Tran, &SliceOnePath> for tran::Voltage {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        to_save: &SliceOnePath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_voltage(SimSignal::ScirVoltage(to_save.clone()))
    }
}

impl Save<Spectre, Tran, &ConvertedNodePath> for tran::Voltage {
    fn save(
        ctx: &SimulationContext<Spectre>,
        to_save: &ConvertedNodePath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(
            ctx,
            match to_save {
                ConvertedNodePath::Cell(path) => path.clone(),
                ConvertedNodePath::Primitive {
                    instances, port, ..
                } => SliceOnePath::new(instances.clone(), NamedSliceOne::new(port.clone())),
            },
            opts,
        )
    }
}

impl Save<Spectre, Tran, &NodePath> for tran::Voltage {
    fn save(
        ctx: &SimulationContext<Spectre>,
        to_save: &NodePath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(ctx, ctx.lib.convert_node_path(to_save).unwrap(), opts)
    }
}

#[impl_dispatch({SliceOnePath; ConvertedNodePath; NodePath})]
impl<T> Save<Spectre, Tran, T> for tran::Voltage {
    fn save(
        ctx: &SimulationContext<Spectre>,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(ctx, &to_save, opts)
    }
}

/// An identifier for a saved transient current.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentKey(pub(crate) Vec<u64>);

impl FromSaved<Spectre, Tran> for tran::Current {
    type Key = CurrentKey;
    fn from_saved(output: &<Tran as Analysis>::Output, key: Self::Key) -> Self {
        let currents: Vec<Arc<Vec<f64>>> = key
            .0
            .iter()
            .map(|key| {
                output
                    .raw_values
                    .get(output.saved_values.get(key).unwrap())
                    .unwrap()
                    .clone()
            })
            .collect();

        let mut total_current = vec![0.; output.time.len()];
        for tran_current in currents {
            for (i, current) in tran_current.iter().enumerate() {
                total_current[i] += *current;
            }
        }
        tran::Current(Arc::new(total_current))
    }
}

#[impl_dispatch({&str; &String; ArcStr; String; SimSignal})]
impl<T> Save<Spectre, Tran, T> for tran::Current {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_current(to_save)
    }
}

impl Save<Spectre, Tran, &SliceOnePath> for tran::Current {
    fn save(
        _ctx: &SimulationContext<Spectre>,
        to_save: &SliceOnePath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        opts.save_tran_current(SimSignal::ScirCurrent(to_save.clone()))
    }
}

impl Save<Spectre, Tran, &ConvertedNodePath> for tran::Current {
    fn save(
        ctx: &SimulationContext<Spectre>,
        to_save: &ConvertedNodePath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(
            ctx,
            match to_save {
                ConvertedNodePath::Cell(path) => path.clone(),
                ConvertedNodePath::Primitive {
                    instances, port, ..
                } => SliceOnePath::new(instances.clone(), NamedSliceOne::new(port.clone())),
            },
            opts,
        )
    }
}

impl Save<Spectre, Tran, &TerminalPath> for tran::Current {
    fn save(
        ctx: &SimulationContext<Spectre>,
        to_save: &TerminalPath,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        CurrentKey(
            ctx.lib
                .convert_terminal_path(to_save)
                .unwrap()
                .into_iter()
                .flat_map(|path| Self::save(ctx, path, opts).0)
                .collect(),
        )
    }
}

#[impl_dispatch({SliceOnePath; ConvertedNodePath; TerminalPath})]
impl<T> Save<Spectre, Tran, T> for tran::Current {
    fn save(
        ctx: &SimulationContext<Spectre>,
        to_save: T,
        opts: &mut <Spectre as Simulator>::Options,
    ) -> Self::Key {
        Self::save(ctx, &to_save, opts)
    }
}

impl Analysis for Tran {
    type Output = Output;
}

impl SupportedBy<Spectre> for Tran {
    fn into_input(self, inputs: &mut Vec<<Spectre as Simulator>::Input>) {
        inputs.push(self.into());
    }
    fn from_output(
        outputs: &mut impl Iterator<Item = <Spectre as Simulator>::Output>,
    ) -> <Self as Analysis>::Output {
        let item = outputs.next().unwrap();
        item.try_into().unwrap()
    }
}
