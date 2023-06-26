use arcstr::ArcStr;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::block::AnalogIo;
use crate::pdk::Pdk;
use crate::{block::Block, schematic::HasSchematic};

use super::{HardwareData, HardwareType, HasSchematicImpl, PrimitiveDevice, Signal};

#[derive(Debug, Clone)]
pub struct ResistorIo {
    pub p: Signal,
    pub n: Signal,
}

#[derive(Debug, Clone)]
pub struct VdividerIo {
    pub vdd: Signal,
    pub vss: Signal,
    pub out: Signal,
}

// AUTOGENERATED CODE BEGIN
impl AnalogIo for ResistorIo {}
impl AnalogIo for VdividerIo {}

impl HardwareType for ResistorIo {
    type Data = ResistorIoData;
    fn num_signals(&self) -> u64 {
        self.p.num_signals() + self.n.num_signals()
    }
    fn instantiate<'n>(&self, ids: &'n [super::Node]) -> (Self::Data, &'n [super::Node]) {
        let (p, ids) = self.p.instantiate(ids);
        let (n, ids) = self.n.instantiate(ids);
        (Self::Data { p, n }, ids)
    }
}

pub struct ResistorIoData {
    pub p: <Signal as HardwareType>::Data,
    pub n: <Signal as HardwareType>::Data,
}

impl HardwareData for ResistorIoData {
    fn flatten(&self) -> Vec<super::Node> {
        vec![self.p, self.n]
    }
    fn flatten_hierarchical(&self) -> Vec<Vec<super::Node>> {
        vec![self.p.flatten(), self.n.flatten()]
    }
}

impl HardwareType for VdividerIo {
    type Data = VdividerIoData;
    fn num_signals(&self) -> u64 {
        self.vdd.num_signals() + self.vss.num_signals() + self.out.num_signals()
    }
    fn instantiate<'n>(&self, ids: &'n [super::Node]) -> (Self::Data, &'n [super::Node]) {
        let (vdd, ids) = self.vdd.instantiate(ids);
        let (vss, ids) = self.vss.instantiate(ids);
        let (out, ids) = self.out.instantiate(ids);
        (Self::Data { vdd, vss, out }, ids)
    }
}

pub struct VdividerIoData {
    pub vdd: <Signal as HardwareType>::Data,
    pub vss: <Signal as HardwareType>::Data,
    pub out: <Signal as HardwareType>::Data,
}

impl HardwareData for VdividerIoData {
    fn flatten(&self) -> Vec<super::Node> {
        vec![self.vdd, self.vss, self.out]
    }
    fn flatten_hierarchical(&self) -> Vec<Vec<super::Node>> {
        vec![self.vdd.flatten(), self.vss.flatten(), self.out.flatten()]
    }
}
// AUTOGENERATED CODE END

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resistor {
    pub r: usize,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vdivider {
    pub r1: Resistor,
    pub r2: Resistor,
}

impl Block for Resistor {
    type Io = ResistorIo;

    fn id() -> ArcStr {
        arcstr::literal!("resistor")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("resistor_{}", self.r)
    }

    fn io(&self) -> Self::Io {
        Self::Io {
            p: Signal,
            n: Signal,
        }
    }
}

impl Block for Vdivider {
    type Io = VdividerIo;

    fn id() -> ArcStr {
        arcstr::literal!("vdivider")
    }

    fn name(&self) -> ArcStr {
        arcstr::format!("vdivider_{}_{}", self.r1.name(), self.r2.name())
    }

    fn io(&self) -> Self::Io {
        Self::Io {
            vdd: Signal,
            vss: Signal,
            out: Signal,
        }
    }
}

impl HasSchematic for Resistor {
    type Data = ();
}

impl HasSchematic for Vdivider {
    type Data = ();
}

impl<PDK: Pdk> HasSchematicImpl<PDK> for Resistor {
    fn schematic(
        &self,
        io: ResistorIoData,
        cell: &mut super::CellBuilder<PDK, Self>,
    ) -> crate::error::Result<Self::Data> {
        cell.add_primitive(PrimitiveDevice::Res2 {
            pos: io.p,
            neg: io.n,
            value: dec!(1000),
        });
        Ok(())
    }
}

impl<PDK: Pdk> HasSchematicImpl<PDK> for Vdivider {
    fn schematic(
        &self,
        io: VdividerIoData,
        cell: &mut super::CellBuilder<PDK, Self>,
    ) -> crate::error::Result<Self::Data> {
        let r1 = cell.instantiate(self.r1);
        let r2 = cell.instantiate(self.r2);

        cell.connect(&io.vdd, &r1.io.p);
        cell.connect(&io.out, &r1.io.n);
        cell.connect(&io.out, &r2.io.p);
        cell.connect(&io.vss, &r2.io.n);
        Ok(())
    }
}
