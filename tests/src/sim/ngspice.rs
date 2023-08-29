use approx::relative_eq;
use ngspice::blocks::Vsource;
use ngspice::tran::{Tran, TranCurrent, TranVoltage};
use ngspice::{Ngspice, NgspicePrimitive, Options};
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use sky130pdk::Sky130OpenPdk;
use substrate::block::{self, Block};
use substrate::io::{SchematicType, Signal, TestbenchIo};
use substrate::pdk::Pdk;
use substrate::schematic::schema::Schema;
use substrate::schematic::{
    Cell, CellBuilder, ExportsNestedNodes, Instance, InstanceData, Schematic, SchematicData,
};
use substrate::simulation::data::{FromSaved, Save};
use substrate::simulation::{SimController, SimulationContext, Simulator, Testbench};
use test_log::test;

use crate::paths::get_path;
use crate::shared::pdk::sky130_open_ctx;

#[test]
fn ngspice_can_save_voltages_and_currents() {
    #[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Block)]
    #[substrate(io = "TestbenchIo", kind = "block::Cell")]
    struct ResistorTb;

    #[derive(SchematicData)]
    struct ResistorTbData {
        r1: InstanceData<ngspice::blocks::Resistor>,
        r2: InstanceData<ngspice::blocks::Resistor>,
        r3: InstanceData<ngspice::blocks::Resistor>,
    }

    impl ExportsNestedNodes for ResistorTb {
        type NestedNodes = ResistorTbData;
    }

    impl Schematic<Sky130OpenPdk, Ngspice> for ResistorTb {
        fn schematic(
            &self,
            io: &<<Self as Block>::Io as SchematicType>::Bundle,
            cell: &mut CellBuilder<Sky130OpenPdk, Ngspice>,
        ) -> substrate::error::Result<Self::NestedNodes> {
            let vdd = cell.signal("vdd", Signal);
            let r1 = cell.instantiate_tb(ngspice::blocks::Resistor(dec!(100)));
            let r2 = cell.instantiate_tb(ngspice::blocks::Resistor(dec!(100)));
            let r3 = cell.instantiate_tb(ngspice::blocks::Resistor(dec!(100)));

            cell.connect(r1.io().p, vdd);
            cell.connect(r1.io().n, r2.io().p);
            cell.connect(r2.io().n, io.vss);
            cell.connect(r1.io().n, r3.io().p);
            cell.connect(r3.io().n, io.vss);

            let vsource = cell.instantiate_tb(Vsource::dc(dec!(1.8)));
            cell.connect(vsource.io().p, vdd);
            cell.connect(vsource.io().n, io.vss);

            Ok(ResistorTbData { r1, r2, r3 })
        }
    }

    #[derive(FromSaved, Serialize, Deserialize)]
    struct ResistorTbOutput {
        r1: TranCurrent,
        r2: TranCurrent,
        r3: TranCurrent,
        vout: TranVoltage,
        r3_terminal: TranCurrent,
    }

    impl<PDK: Pdk> Save<Ngspice, Tran, &Cell<NgspicePrimitive, ResistorTb>> for ResistorTbOutput {
        fn save(
            ctx: &SimulationContext<Ngspice>,
            to_save: &Cell<NgspicePrimitive, ResistorTb>,
            opts: &mut <Ngspice as Simulator>::Options,
        ) -> Self::Key {
            Self::Key {
                r1: TranCurrent::save(ctx, to_save.data().r1, opts),
                r2: TranCurrent::save(ctx, to_save.data().r2, opts),
                r3: TranCurrent::save(ctx, to_save.data().r3, opts),
                vout: TranVoltage::save(ctx, to_save.data().r1.terminals().n, opts),
                r3_terminal: TranCurrent::save(ctx, to_save.data().r3.terminals().p, opts),
            }
        }
    }

    impl Testbench<Sky130OpenPdk, Ngspice> for ResistorTb {
        type Output = ResistorTbOutput;

        fn run(&self, sim: SimController<Sky130OpenPdk, Ngspice, Self>) -> Self::Output {
            sim.simulate(
                Options::default(),
                None,
                Tran {
                    step: dec!(2e-10),
                    stop: dec!(2e-9),
                    ..Default::default()
                },
            )
            .expect("failed to run simulation")
        }
    }

    let test_name = "ngspice_can_save_voltages_and_currents";
    let sim_dir = get_path(test_name, "sim/");
    let ctx = sky130_open_ctx();
    let ResistorTbOutput {
        r1,
        r2,
        r3,
        vout,
        r3_terminal,
    } = ctx.simulate(ResistorTb, sim_dir).unwrap();

    for (actual, expected) in [
        (&*r1, 1.8 / 150.),
        (&*r2, 1.8 / 300.),
        (&*r3, 1.8 / 300.),
        (&*vout, 1.8 / 3.),
        (&*r3_terminal, -1.8 / 300.),
    ] {
        assert!(actual
            .iter()
            .cloned()
            .all(|val| relative_eq!(val, expected)));
    }
}
