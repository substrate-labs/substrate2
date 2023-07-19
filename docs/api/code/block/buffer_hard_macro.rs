#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Block, HasSchematicImpl)]
#[substrate(io = "BufferIo")]
#[substrate(schematic(
    source = "r###\"
        * CMOS buffer

        .subckt buffer din dout vdd vss
        X0 din dinb vdd vss inverter
        X1 dinb dout vdd vss inverter
        .ends

        .subckt inverter din dout vdd vss
        X0 dout din vss vss sky130_fd_pr__nfet_01v8 w=2 l=0.15
        X1 dout din vdd vdd sky130_fd_pr__pfet_01v8 w=4 l=0.15
        .ends
    \"###",
    name = "buffer",
    fmt = "inline-spice",
    pdk = "Sky130OpenPdk"
))]
pub struct BufferInlineHardMacro;
