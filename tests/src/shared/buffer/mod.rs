use serde::{Deserialize, Serialize};

use substrate::io::{Array, CustomLayoutType, InOut, Input, LayoutPort, Output, ShapePort, Signal};
use substrate::{Io, LayoutType};

use substrate::block::Block;

pub mod layout;
pub mod schematic;

#[derive(Io, Clone, Default)]
pub struct BufferIo {
    #[io(layout_type = "ShapePort")]
    pub vdd: InOut<Signal>,
    #[io(layout_type = "ShapePort")]
    pub vss: InOut<Signal>,
    #[io(layout_type = "ShapePort")]
    pub din: Input<Signal>,
    #[io(layout_type = "ShapePort")]
    pub dout: Output<Signal>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Inverter {
    strength: usize,
}

impl Inverter {
    pub fn new(strength: usize) -> Self {
        Self { strength }
    }
}

impl Block for Inverter {
    type Io = BufferIo;

    fn id() -> arcstr::ArcStr {
        arcstr::literal!("inverter")
    }

    fn name(&self) -> arcstr::ArcStr {
        arcstr::format!("inverter_{}", self.strength)
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Buffer {
    strength: usize,
}

impl Buffer {
    pub fn new(strength: usize) -> Self {
        Self { strength }
    }
}

impl Block for Buffer {
    type Io = BufferIo;

    fn id() -> arcstr::ArcStr {
        arcstr::literal!("buffer")
    }

    fn name(&self) -> arcstr::ArcStr {
        arcstr::format!("buffer_{}", self.strength)
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

#[derive(Io, Clone, Default)]
#[io(layout_type = "BufferNIoLayout")]
pub struct BufferNIo {
    pub vdd: InOut<Signal>,
    pub vss: InOut<Signal>,
    pub din: Input<Signal>,
    pub dout: Output<Signal>,
}

#[derive(LayoutType, Clone)]
pub struct BufferNIoLayout {
    pub vdd: LayoutPort,
    pub vss: LayoutPort,
    pub din: ShapePort,
    pub dout: ShapePort,
}

impl CustomLayoutType<BufferNIo> for BufferNIoLayout {
    fn from_layout_type(_other: &BufferNIo) -> Self {
        Self {
            vdd: LayoutPort,
            vss: LayoutPort,
            din: ShapePort,
            dout: ShapePort,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BufferN {
    strength: usize,
    n: usize,
}

impl BufferN {
    pub fn new(strength: usize, n: usize) -> Self {
        Self { strength, n }
    }
}

impl Block for BufferN {
    type Io = BufferNIo;

    fn id() -> arcstr::ArcStr {
        arcstr::literal!("buffer_n")
    }

    fn name(&self) -> arcstr::ArcStr {
        arcstr::format!("buffer_{}_{}", self.strength, self.n)
    }

    fn io(&self) -> Self::Io {
        Default::default()
    }
}

#[derive(Io, Clone)]
pub struct BufferNxMIo {
    vdd: InOut<Signal>,
    vss: InOut<Signal>,
    #[io(layout_type = "Array<ShapePort>")]
    din: Input<Array<Signal>>,
    #[io(layout_type = "Array<ShapePort>")]
    dout: Output<Array<Signal>>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BufferNxM {
    strength: usize,
    n: usize,
    m: usize,
}

impl BufferNxM {
    pub fn new(strength: usize, n: usize, m: usize) -> Self {
        Self { strength, n, m }
    }
}

impl Block for BufferNxM {
    type Io = BufferNxMIo;

    fn id() -> arcstr::ArcStr {
        arcstr::literal!("buffer_n_m")
    }

    fn name(&self) -> arcstr::ArcStr {
        arcstr::format!("buffer_{}_{}x{}", self.strength, self.n, self.m)
    }

    fn io(&self) -> Self::Io {
        Self::Io {
            din: Input(Array::new(self.m, Default::default())),
            dout: Output(Array::new(self.m, Default::default())),
            vdd: Default::default(),
            vss: Default::default(),
        }
    }
}
