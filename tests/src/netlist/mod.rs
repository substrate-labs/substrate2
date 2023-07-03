use rust_decimal_macros::dec;
use scir::*;
use spice::Netlister;

/// Creates a 1:3 resistive voltage divider.
pub(crate) fn vdivider() -> Library {
    let mut lib = Library::new("vdivider");
    let mut wrapper = Cell::new_whitebox("resistor_wrapper");
    let pos = wrapper.add_node("pos");
    let neg = wrapper.add_node("neg");
    let contents = wrapper.contents_mut().as_mut().unwrap_clear();
    contents.add_primitive(PrimitiveDevice::Res2 {
        pos,
        neg,
        value: dec!(3300).into(),
    });
    wrapper.expose_port(pos);
    wrapper.expose_port(neg);
    let wrapper = lib.add_cell(wrapper);

    let mut vdivider = Cell::new_whitebox("vdivider");
    let vdd = vdivider.add_node("vdd");
    let out = vdivider.add_node("out");
    let int = vdivider.add_node("int");
    let vss = vdivider.add_node("vss");

    let contents = vdivider.contents_mut().as_mut().unwrap_clear();
    let mut r1 = Instance::new("r1", wrapper);
    r1.connect("pos", vdd);
    r1.connect("neg", int);
    contents.add_instance(r1);

    let mut r2 = Instance::new("r2", wrapper);
    r2.connect("pos", int);
    r2.connect("neg", out);
    contents.add_instance(r2);

    let mut r3 = Instance::new("r3", wrapper);
    r3.connect("pos", out);
    r3.connect("neg", vss);
    contents.add_instance(r3);

    vdivider.expose_port(vdd);
    vdivider.expose_port(vss);
    vdivider.expose_port(out);
    lib.add_cell(vdivider);

    lib
}

#[test]
fn vdivider_is_valid() {
    let lib = vdivider();
    let issues = lib.validate();
    assert_eq!(issues.num_errors(), 0);
    assert_eq!(issues.num_warnings(), 0);
}

#[test]
fn netlist_spice() {
    let lib = vdivider();
    let mut buf: Vec<u8> = Vec::new();
    let netlister = Netlister::new(&lib, &mut buf);
    netlister.export().unwrap();
    let string = String::from_utf8(buf).unwrap();
    println!("{}", string);

    // TODO: more robust assertions about the output
    // Once we have a SPICE parser, we can parse the SPICE back to SCIR
    // and assert that the input SCIR is equivalent to the output SCIR.

    assert_eq!(string.matches("SUBCKT").count(), 2);
    assert_eq!(string.matches("ENDS").count(), 2);
    assert_eq!(string.matches("Xr1").count(), 1);
    assert_eq!(string.matches("Xr2").count(), 1);
    assert_eq!(string.matches("Xr3").count(), 1);
    assert_eq!(string.matches("resistor_wrapper").count(), 5);
    assert_eq!(string.matches("vdivider").count(), 3);
    assert_eq!(string.matches("* vdivider").count(), 1);
    assert_eq!(string.matches("R0 pos neg 3300").count(), 1);
}

#[test]
fn netlist_spectre() {
    let lib = vdivider();
    let mut buf: Vec<u8> = Vec::new();
    let netlister = spectre::netlist::Netlister::new(&lib, &mut buf);
    netlister.export().unwrap();
    let string = String::from_utf8(buf).unwrap();
    println!("{}", string);

    // TODO: more robust assertions about the output
    // Once we have a Spectre netlist parser, we can parse the Spectre back to SCIR
    // and assert that the input SCIR is equivalent to the output SCIR.

    assert_eq!(string.matches("subckt").count(), 2);
    assert_eq!(string.matches("ends").count(), 2);
    assert_eq!(string.matches("r1").count(), 1);
    assert_eq!(string.matches("r2").count(), 1);
    assert_eq!(string.matches("r3").count(), 1);
    assert_eq!(string.matches("resistor_wrapper").count(), 5);
    assert_eq!(string.matches("vdivider").count(), 3);
    assert_eq!(string.matches("// vdivider").count(), 1);
    assert_eq!(
        string.matches("res0 ( pos neg ) resistor r=3300").count(),
        1
    );
}
