use crate::corner::Sky130Corner;
use crate::layout::to_gds;
use crate::mos::{MosParams, Nfet01v8};
use crate::stdcells::And2;
use crate::Sky130Pdk;
use approx::assert_abs_diff_eq;
use derive_where::derive_where;
use gds::GdsUnits;
use gdsconv::export::GdsExportOpts;
use ngspice::blocks::Vsource;
use ngspice::Ngspice;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use spectre::analysis::montecarlo::Variations;
use spectre::Spectre;
use std::any::Any;
use std::marker::PhantomData;
use std::path::PathBuf;
use substrate::block::Block;
use substrate::context::Context;
use substrate::schematic::schema::{FromSchema, Schema};
use substrate::schematic::{Cell, CellBuilder, Schematic};
use substrate::simulation::{SimController, SimulationContext, Simulator, Testbench};
use substrate::types::schematic::Terminal;
use substrate::types::{Signal, TestbenchIo, TwoTerminalIo};

const BUILD_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/build");

#[inline]
pub(crate) fn get_path(test_name: &str, file_name: &str) -> PathBuf {
    PathBuf::from(BUILD_DIR).join(test_name).join(file_name)
}

/// Create a new Substrate context for the SKY130 open-source PDK.
///
/// Sets the PDK root to the value of the `SKY130_OPEN_PDK_ROOT`
/// environment variable.
///
/// # Panics
///
/// Panics if the `SKY130_OPEN_PDK_ROOT` environment variable is not set,
/// or if the value of that variable is not a valid UTF-8 string.
pub fn sky130_open_ctx() -> Context {
    let pdk_root = std::env::var("SKY130_OPEN_PDK_ROOT")
        .expect("the SKY130_OPEN_PDK_ROOT environment variable must be set");
    Context::builder()
        .install(Ngspice::default())
        .install(Sky130Pdk::open(pdk_root))
        .build()
}

/// Create a new Substrate context for the SKY130 commercial PDK.
///
/// Sets the PDK root to the value of the `SKY130_COMMERCIAL_PDK_ROOT`
/// environment variable and installs Spectre with default configuration.
///
/// # Panics
///
/// Panics if the `SKY130_COMMERCIAL_PDK_ROOT` environment variable is not set,
/// or if the value of that variable is not a valid UTF-8 string.
pub fn sky130_commercial_ctx() -> Context {
    // Open PDK needed for standard cells.
    let open_pdk_root = std::env::var("SKY130_OPEN_PDK_ROOT")
        .expect("the SKY130_OPEN_PDK_ROOT environment variable must be set");
    let commercial_pdk_root = std::env::var("SKY130_COMMERCIAL_PDK_ROOT")
        .expect("the SKY130_COMMERCIAL_PDK_ROOT environment variable must be set");
    Context::builder()
        .install(Spectre::default())
        .install(Sky130Pdk::new(open_pdk_root, commercial_pdk_root))
        .build()
}

#[derive_where(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct And2Tb<S> {
    schema: PhantomData<fn() -> S>,
    vdd: Decimal,
    a: Decimal,
    b: Decimal,
}

impl<S: Any> Block for And2Tb<S> {
    type Io = TestbenchIo;

    fn name(&self) -> arcstr::ArcStr {
        arcstr::literal!("and2_tb")
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

pub trait SupportsAnd2Tb: FromSchema<Sky130Pdk> {
    type DcVsource: Block<Io = TwoTerminalIo> + Schematic<Schema = Self>;

    fn dc_vsource(v: Decimal) -> Self::DcVsource;
}

impl SupportsAnd2Tb for Ngspice {
    type DcVsource = ngspice::blocks::Vsource;
    fn dc_vsource(v: Decimal) -> Self::DcVsource {
        ngspice::blocks::Vsource::dc(v)
    }
}

impl SupportsAnd2Tb for Spectre {
    type DcVsource = spectre::blocks::Vsource;
    fn dc_vsource(v: Decimal) -> Self::DcVsource {
        spectre::blocks::Vsource::dc(v)
    }
}

impl<S: SupportsAnd2Tb> Schematic for And2Tb<S> {
    type Schema = S;
    type NestedData = Terminal;
    fn schematic(
        &self,
        io: &substrate::types::schematic::IoNodeBundle<Self>,
        cell: &mut CellBuilder<<Self as Schematic>::Schema>,
    ) -> substrate::error::Result<Self::NestedData> {
        let vddsrc = cell.instantiate(S::dc_vsource(self.vdd));
        let asrc = cell.instantiate(S::dc_vsource(self.a));
        let bsrc = cell.instantiate(S::dc_vsource(self.b));
        let and2 = cell
            .sub_builder::<Sky130Pdk>()
            .instantiate_blocking(And2::S0)
            .unwrap();

        cell.connect(io.vss, vddsrc.io().n);
        cell.connect_multiple(&[
            vddsrc.io().n,
            asrc.io().n,
            bsrc.io().n,
            and2.io().pwr.vgnd,
            and2.io().pwr.vnb,
        ]);
        cell.connect_multiple(&[vddsrc.io().p, and2.io().pwr.vpwr, and2.io().pwr.vpb]);
        cell.connect(and2.io().a, asrc.io().p);
        cell.connect(and2.io().b, bsrc.io().p);

        Ok(and2.io().x)
    }
}

#[test]
fn sky130_and2_ngspice() {
    let test_name = "sky130_and2_ngspice";
    let sim_dir = get_path(test_name, "sim/");
    let ctx = sky130_open_ctx();

    for (a, b, expected) in [(dec!(1.8), dec!(1.8), 1.8f64), (dec!(1.8), dec!(0), 0f64)] {
        let mut sim = ctx
            .get_sim_controller(
                And2Tb {
                    schema: PhantomData,
                    vdd: dec!(1.8),
                    a,
                    b,
                },
                &sim_dir,
            )
            .expect("failed to create sim controller");
        let mut opts = ngspice::Options::default();
        sim.set_option(Sky130Corner::Tt, &mut opts);
        let vout = sim
            .simulate(
                opts,
                ngspice::tran::Tran {
                    step: dec!(1e-9),
                    stop: dec!(2e-9),
                    ..Default::default()
                },
            )
            .expect("failed to run simulation");
        assert_abs_diff_eq!(*vout.v.last().unwrap(), expected, epsilon = 1e-6);
    }
}

#[cfg(feature = "spectre")]
#[test]
fn sky130_and2_monte_carlo_spectre() {
    let test_name = "sky130_and2_spectre";
    let sim_dir = get_path(test_name, "sim/");
    let ctx = sky130_commercial_ctx();

    for (a, b, expected) in [
        (dec!(1.8), dec!(1.8), 1.8f64),
        (dec!(1.8), dec!(0), 0f64),
        (dec!(0), dec!(1.8), 0f64),
        (dec!(0), dec!(0), 0f64),
    ] {
        let mut sim = ctx
            .get_sim_controller(
                And2Tb {
                    schema: PhantomData,
                    vdd: dec!(1.8),
                    a,
                    b,
                },
                &sim_dir,
            )
            .expect("failed to create sim controller");
        let mut opts = spectre::Options::default();
        sim.set_option(Sky130Corner::Tt, &mut opts);
        let mc_vout = sim
            .simulate(
                opts,
                spectre::analysis::montecarlo::MonteCarlo {
                    variations: Variations::All,
                    numruns: 4,
                    seed: None,
                    firstrun: None,
                    analysis: spectre::analysis::tran::Tran {
                        stop: dec!(2e-9),
                        errpreset: Some(spectre::ErrPreset::Conservative),
                        ..Default::default()
                    },
                },
            )
            .expect("failed to run simulation");
        assert_eq!(
            mc_vout.len(),
            4,
            "MonteCarlo output did not contain data from the correct number of runs"
        );
        for vout in &*mc_vout {
            assert_abs_diff_eq!(*vout.v.last().unwrap(), expected, epsilon = 1e-6);
        }
    }
}

#[test]
fn nfet_01v8_layout() {
    let test_name = "nfet_01v8_layout";
    let ctx = sky130_commercial_ctx();
    let layout_path = get_path(test_name, "layout.gds");

    let layir = ctx
        .export_layir(Nfet01v8::new(MosParams {
            w: 2_400,
            l: 150,
            nf: 1,
        }))
        .unwrap();
    let layir = to_gds(&layir.layir);
    let gds = gdsconv::export::export_gds(
        layir,
        GdsExportOpts {
            name: arcstr::literal!("nfet_01v8_layout"),
            units: Some(GdsUnits::new(1., 1e-9)),
        },
    );
    gds.save(layout_path).unwrap();
}
