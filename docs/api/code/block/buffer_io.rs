#[derive(Io, Clone, Default)]
pub struct BufferIo {
    vdd: InOut<Signal>,
    vss: InOut<Signal>,
    #[io(layout_type = "ShapePort")]
    din: Input<Signal>,
    #[io(layout_type = "ShapePort")]
    dout: Output<Signal>,
}
