//! Interfaces for interacting with simulation data.
use crate::{
    simulation::{Analysis, SimulationContext, Simulator},
    types::{
        schematic::{HasNestedView, Nested},
        HasView,
    },
};

/// Saves the raw output of a simulation.
#[derive(Debug, Clone, Copy)]
pub struct SaveOutput;
/// Saves the transient time waveform.
#[derive(Debug, Clone, Copy)]
pub struct SaveTime;

impl HasView<Nested> for SaveOutput {
    type View = SaveOutput;
}

impl HasNestedView for SaveOutput {
    type NestedView = SaveOutput;

    fn nested_view(&self, _parent: &substrate::schematic::InstancePath) -> Self::NestedView {
        *self
    }
}

impl HasView<Nested> for SaveTime {
    type View = SaveTime;
}

impl HasNestedView for SaveTime {
    type NestedView = SaveTime;

    fn nested_view(&self, _parent: &substrate::schematic::InstancePath) -> Self::NestedView {
        *self
    }
}

/// Gets the [`Save::SaveKey`] corresponding to type `T`.
pub type SaveKey<T, S, A> = <T as Save<S, A>>::SaveKey;

/// A schematic object that can be saved in an analysis within a given simulator.
pub trait Save<S: Simulator, A: Analysis> {
    /// The key type used to address the saved output within the analysis.
    ///
    /// This key is assigned in [`Save::save`].
    type SaveKey;
    /// The saved data associated with things object.
    type Save;

    /// Marks the given output for saving, returning a key that can be used to recover
    /// the output once the simulation is complete.
    fn save(
        &self,
        ctx: &SimulationContext<S>,
        opts: &mut <S as Simulator>::Options,
    ) -> <Self as Save<S, A>>::SaveKey;

    /// Recovers the desired simulation output from the analysis's output.
    fn from_saved(
        output: &<A as Analysis>::Output,
        key: &<Self as Save<S, A>>::SaveKey,
    ) -> <Self as Save<S, A>>::Save;
}
