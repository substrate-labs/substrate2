#[derive(Io, Clone, Default)]
#[substrate(layout_type = "BufferIoLayout")]
pub struct BufferIo {
    vdd: InOut<Signal>,
    vss: InOut<Signal>,
    din: Input<Signal>,
    dout: Output<Signal>,
}

#[derive(LayoutType, Clone)]
pub struct BufferIoLayout {
    vdd: LayoutPort,
    vss: LayoutPort,
    din: ShapePort,
    dout: ShapePort,
}

impl CustomLayoutType<BufferIo> for BufferIoLayout {
    fn from_layout_type(_other: &BufferIo) -> Self {
        Self {
            vdd: LayoutPort,
            vss: LayoutPort,
            din: ShapePort,
            dout: ShapePort,
        }
    }
}
