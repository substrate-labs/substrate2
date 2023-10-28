use itertools::Itertools;
use std::io::Write;
use test_log::test;

use crate::netlist::{HasSpiceLikeNetlist, Include, NetlistKind, NetlisterInstance, RenameGround};
use crate::schema::{FromSchema, StringSchema};
use crate::*;

#[test]
fn duplicate_cell_names() {
    let c1 = Cell::new("duplicate_cell_name");
    let c2 = Cell::new("duplicate_cell_name");
    let mut lib = <LibraryBuilder>::new();
    lib.add_cell(c1);
    lib.add_cell(c2);
    let issues = lib.validate();
    assert!(issues.has_error());
}

#[test]
fn duplicate_instance_names() {
    let mut lib = LibraryBuilder::<StringSchema>::new();
    let id = lib.add_primitive("res".into());

    let mut vdivider = Cell::new("vdivider");
    let vdd = vdivider.add_node("vdd");
    let out = vdivider.add_node("out");
    let int = vdivider.add_node("int");
    let vss = vdivider.add_node("vss");

    let mut r1 = Instance::new("r1", id);
    r1.connect("1", vdd);
    r1.connect("2", int);
    vdivider.add_instance(r1);

    // Duplicate instance name
    let mut r2 = Instance::new("r1", id);
    r2.connect("1", int);
    r2.connect("2", out);
    vdivider.add_instance(r2);

    vdivider.expose_port(vdd, Direction::InOut);
    vdivider.expose_port(vss, Direction::InOut);
    vdivider.expose_port(out, Direction::Output);

    lib.add_cell(vdivider);

    let issues = lib.validate();
    assert!(issues.has_error());
}

#[test]
fn duplicate_signal_names() {
    let mut lib = LibraryBuilder::<StringSchema>::new();

    let mut cell = Cell::new("cell");
    cell.add_node("duplicate_signal");
    cell.add_node("duplicate_signal");
    lib.add_cell(cell);

    let issues = lib.validate();
    assert!(issues.has_error());
}

#[test]
fn no_schema_conversion() {
    let mut lib = LibraryBuilder::<StringSchema>::new();
    let empty_cell = Cell::new("empty");
    let id = lib.add_cell(empty_cell);

    let no_schema_lib = lib.drop_schema().unwrap();
    assert_eq!(no_schema_lib.cell(id).name(), "empty");

    let mut lib: LibraryBuilder<StringSchema> = no_schema_lib.convert_schema().unwrap();
    assert_eq!(lib.cell(id).name(), "empty");

    let id = lib.add_primitive("res".into());

    let mut resistor = Cell::new("resistor");
    let vdd = resistor.add_node("vdd");
    let vss = resistor.add_node("vss");

    let mut r1 = Instance::new("r1", id);
    r1.connect("1", vdd);
    r1.connect("2", vss);
    resistor.add_instance(r1);

    resistor.expose_port(vdd, Direction::InOut);
    resistor.expose_port(vss, Direction::InOut);

    lib.add_cell(resistor);

    assert!(lib.drop_schema().is_err());
}

#[test]
fn schema_conversion() {
    pub struct PartiallyTypedSchema;

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum PartiallyTypedPrimitive {
        PrimA,
        PrimB,
        Other(ArcStr),
    }

    impl Schema for PartiallyTypedSchema {
        type Primitive = PartiallyTypedPrimitive;
    }

    impl FromSchema<StringSchema> for PartiallyTypedSchema {
        type Error = ();

        fn convert_primitive(
            primitive: <StringSchema as Schema>::Primitive,
        ) -> Result<<Self as Schema>::Primitive, Self::Error> {
            Ok(match primitive.as_ref() {
                "prim_a" => PartiallyTypedPrimitive::PrimA,
                "prim_b" => PartiallyTypedPrimitive::PrimB,
                "invalid_prim" => {
                    return Err(());
                }
                _ => PartiallyTypedPrimitive::Other(primitive),
            })
        }

        fn convert_instance(
            instance: &mut Instance,
            primitive: &<StringSchema as Schema>::Primitive,
        ) -> Result<(), Self::Error> {
            instance.map_connections(|conn| match (primitive.as_ref(), conn.as_ref()) {
                ("prim_a", "a1") => arcstr::literal!("a_pt1"),
                ("prim_a", "a2") => arcstr::literal!("a_pt2"),
                ("prim_b", "b1") => arcstr::literal!("b_pt1"),
                ("prim_b", "b2") => arcstr::literal!("b_pt2"),
                _ => conn,
            });
            Ok(())
        }
    }

    impl FromSchema<PartiallyTypedSchema> for StringSchema {
        type Error = ();

        fn convert_primitive(
            primitive: <PartiallyTypedSchema as Schema>::Primitive,
        ) -> Result<<Self as Schema>::Primitive, Self::Error> {
            Ok(match primitive {
                PartiallyTypedPrimitive::PrimA => arcstr::literal!("prim_a"),
                PartiallyTypedPrimitive::PrimB => arcstr::literal!("prim_b"),
                PartiallyTypedPrimitive::Other(inner) => inner,
            })
        }

        fn convert_instance(
            instance: &mut Instance,
            primitive: &<PartiallyTypedSchema as Schema>::Primitive,
        ) -> Result<(), Self::Error> {
            instance.map_connections(|conn| match (primitive, conn.as_ref()) {
                (&PartiallyTypedPrimitive::PrimA, "a_pt1") => arcstr::literal!("a1"),
                (&PartiallyTypedPrimitive::PrimA, "a_pt2") => arcstr::literal!("a2"),
                (&PartiallyTypedPrimitive::PrimB, "b_pt1") => arcstr::literal!("b1"),
                (&PartiallyTypedPrimitive::PrimB, "b_pt2") => arcstr::literal!("b2"),
                _ => conn,
            });
            Ok(())
        }
    }

    let mut lib = LibraryBuilder::<StringSchema>::new();

    let prim_a = lib.add_primitive("prim_a".into());
    let prim_b = lib.add_primitive("prim_b".into());

    let mut cell = Cell::new("prim_cell");
    let vdd = cell.add_node("vdd");
    let vss = cell.add_node("vss");

    let mut inst_a = Instance::new("inst_a", prim_a);
    inst_a.connect("a1", vdd);
    inst_a.connect("a2", vss);
    let inst_a = cell.add_instance(inst_a);

    let mut inst_b = Instance::new("inst_b", prim_b);
    inst_b.connect("b1", vdd);
    inst_b.connect("b2", vss);
    let b_inst = cell.add_instance(inst_b);

    cell.expose_port(vdd, Direction::InOut);
    cell.expose_port(vss, Direction::InOut);

    let cell = lib.add_cell(cell);

    let ptlib = lib.convert_schema::<PartiallyTypedSchema>().unwrap();
    let ptcell = ptlib.cell(cell);
    assert!(ptcell.instance(inst_a).connections().contains_key("a_pt1"));
    assert!(ptcell.instance(inst_a).connections().contains_key("a_pt2"));
    assert!(ptcell.instance(b_inst).connections().contains_key("b_pt1"));
    assert!(ptcell.instance(b_inst).connections().contains_key("b_pt2"));

    assert_eq!(ptlib.primitive(prim_a), &PartiallyTypedPrimitive::PrimA);
    assert_eq!(ptlib.primitive(prim_b), &PartiallyTypedPrimitive::PrimB);

    let mut orig_lib = ptlib.convert_schema::<StringSchema>().unwrap();
    let orig_cell = orig_lib.cell(cell);
    assert!(orig_cell.instance(inst_a).connections().contains_key("a1"));
    assert!(orig_cell.instance(inst_a).connections().contains_key("a2"));
    assert!(orig_cell.instance(b_inst).connections().contains_key("b1"));
    assert!(orig_cell.instance(b_inst).connections().contains_key("b2"));

    assert_eq!(orig_lib.primitive(prim_a), "prim_a");
    assert_eq!(orig_lib.primitive(prim_b), "prim_b");

    orig_lib.add_primitive("invalid_prim".into());
    assert!(orig_lib.convert_schema::<PartiallyTypedSchema>().is_err());
}

/// Returns a SCIR library with nested cells and 3 varieties of [`SliceOnePath`]s that
/// address the VDD node of the innermost instance for testing purposes.
fn nested_lib(n: usize) -> (Library<StringSchema>, Vec<SliceOnePath>) {
    let mut lib = LibraryBuilder::<StringSchema>::new();

    let prim_inst = lib.add_primitive("prim_inst".into());

    let mut signals = Vec::<(SliceOne, SliceOne)>::new();
    let mut insts = Vec::<InstanceId>::new();
    let mut cells = Vec::<CellId>::new();

    for i in 0..n {
        let mut cell = Cell::new(format!("cell_{}", i));
        let vdd = cell.add_node("vdd");
        let vss = cell.add_node("vss");
        signals.push((vdd, vss));

        let mut inst = Instance::new(
            "inst",
            if i == 0 {
                ChildId::from(prim_inst)
            } else {
                ChildId::from(*cells.last().unwrap())
            },
        );
        if i < n - 1 {
            inst.connect("vdd", vdd);
        }
        inst.connect("vss", vss);
        insts.push(cell.add_instance(inst));

        // Do not expose VDD on topmost two cells to test path simplification.
        if i < n - 2 {
            cell.expose_port(vdd, Direction::InOut);
        }
        cell.expose_port(vss, Direction::InOut);
        cells.push(lib.add_cell(cell));
    }

    let lib = lib.build().unwrap();

    // Test name path API.
    let mut name_path = InstancePath::new(format!("cell_{}", n - 1));
    name_path.push_iter((1..n).map(|_| "inst"));
    let name_path = name_path.slice_one(NamedSliceOne::new("vdd"));

    // Test ID path API.
    let mut id_path = InstancePath::new(*cells.last().unwrap());
    id_path.push_iter((1..n).rev().map(|i| insts[i]));
    let id_path = id_path.slice_one(signals.first().unwrap().0);

    // Test mixing name and ID path APIs.
    let mut mixed_path = InstancePath::new(format!("cell_{}", n - 1));
    mixed_path.push_iter((1..n).rev().map(|i| {
        if i % 2 == 0 {
            InstancePathElement::from(insts[i])
        } else {
            InstancePathElement::from("inst")
        }
    }));
    let mixed_path = mixed_path.slice_one(signals.first().unwrap().0);

    (lib, vec![name_path, id_path, mixed_path])
}

#[test]
fn path_simplification() {
    const N: usize = 5;

    let (lib, paths) = nested_lib(N);

    for path in paths {
        assert_eq!(path.instances().len(), N - 1);
        let simplified_path = lib.simplify_path(path);
        // Simplified path should bubble up to `cell_{N-2}`.
        assert_eq!(simplified_path.instances().len(), 1);
    }
}

#[test]
fn name_path_conversion() {
    const N: usize = 5;

    let (lib, paths) = nested_lib(N);

    for path in paths {
        let name_path = lib.convert_slice_one_path(path, |name, index| {
            if let Some(index) = index {
                arcstr::format!("{}[{}]", name, index)
            } else {
                name.clone()
            }
        });

        assert_eq!(
            name_path.join("."),
            ["inst"; N - 1]
                .into_iter()
                .chain(["vdd"])
                .collect::<Vec<&str>>()
                .join(".")
        );
    }
}

#[test]
fn spice_like_netlist() {
    pub struct SpiceLikeSchema {
        bus_delimiter: (char, char),
    }

    impl Schema for SpiceLikeSchema {
        type Primitive = ArcStr;
    }

    impl HasSpiceLikeNetlist for SpiceLikeSchema {
        fn write_include<W: Write>(&self, out: &mut W, include: &Include) -> std::io::Result<()> {
            if let Some(section) = &include.section {
                write!(out, ".LIB {:?} {}", include.path, section)?;
            } else {
                write!(out, ".INCLUDE {:?}", include.path)?;
            }
            Ok(())
        }

        fn write_start_subckt<W: Write>(
            &self,
            out: &mut W,
            name: &ArcStr,
            ports: &[&SignalInfo],
        ) -> std::io::Result<()> {
            let (start, end) = self.bus_delimiter;
            write!(out, ".SUBCKT {}", name)?;
            for sig in ports {
                if let Some(width) = sig.width {
                    for i in 0..width {
                        write!(out, " {}{}{}{}", sig.name, start, i, end)?;
                    }
                } else {
                    write!(out, " {}", sig.name)?;
                }
            }
            Ok(())
        }

        fn write_end_subckt<W: Write>(&self, out: &mut W, name: &ArcStr) -> std::io::Result<()> {
            write!(out, ".ENDS {}", name)
        }

        fn write_slice<W: Write>(
            &self,
            out: &mut W,
            slice: Slice,
            info: &SignalInfo,
        ) -> std::io::Result<()> {
            let (start, end) = self.bus_delimiter;
            if let Some(range) = slice.range() {
                for i in range.indices() {
                    if i > range.start() {
                        write!(out, " ")?;
                    }
                    write!(out, "{}{}{}{}", &info.name, start, i, end)?;
                }
            } else {
                write!(out, "{}", &info.name)?;
            }
            Ok(())
        }

        fn write_instance<W: Write>(
            &self,
            out: &mut W,
            name: &ArcStr,
            connections: impl Iterator<Item = ArcStr>,
            child: &ArcStr,
        ) -> std::io::Result<ArcStr> {
            write!(out, "{}", name)?;

            for connection in connections {
                write!(out, " {}", connection)?;
            }

            write!(out, " {}", child)?;

            Ok(name.clone())
        }

        fn write_primitive_inst<W: Write>(
            &self,
            out: &mut W,
            name: &ArcStr,
            connections: HashMap<ArcStr, impl Iterator<Item = ArcStr>>,
            primitive: &<Self as Schema>::Primitive,
        ) -> std::io::Result<ArcStr> {
            write!(out, "{}", name)?;

            let connections = connections
                .into_iter()
                .sorted_by_key(|(name, _)| name.clone())
                .collect::<Vec<_>>();

            for (_, connection) in connections {
                for signal in connection {
                    write!(out, " {}", signal)?;
                }
            }

            write!(out, " {}", primitive)?;

            Ok(name.clone())
        }
    }

    const N: usize = 3;

    let mut lib = LibraryBuilder::<SpiceLikeSchema>::new();

    let resistor = lib.add_primitive("resistor".into());

    let mut dut = Cell::new("dut");

    let p = dut.add_bus("p", N);
    let n = dut.add_bus("n", N);

    for i in 0..N {
        let mut resistor = Instance::new(format!("inst_{i}"), resistor);
        resistor.connect("p", p.index(i));
        resistor.connect("n", n.index(i));
        dut.add_instance(resistor);
    }

    dut.expose_port(p, Direction::InOut);
    dut.expose_port(n, Direction::InOut);

    let dut = lib.add_cell(dut);

    let mut tb = Cell::new("tb");

    let vdd = tb.add_node("vdd");
    let vss = tb.add_node("vss");

    let mut dut = Instance::new("dut", dut);
    dut.connect("p", Concat::new(vec![vdd.into(); 3]));
    dut.connect("n", Concat::new(vec![vss.into(); 3]));
    tb.add_instance(dut);

    tb.expose_port(vss, Direction::InOut);
    let tb = lib.add_cell(tb);

    lib.set_top(tb);

    let lib = lib.build().unwrap();

    let schema = SpiceLikeSchema {
        bus_delimiter: ('<', '|'),
    };
    let mut buf = Vec::new();
    let netlister = NetlisterInstance::new(
        NetlistKind::Testbench(RenameGround::Yes("0".into())),
        &schema,
        &lib,
        &[],
        &mut buf,
    );

    netlister.export().unwrap();

    let netlist = std::str::from_utf8(&buf).unwrap();

    println!("{:?}", netlist);
    for fragment in [
        "* netlist_lib",
        r#".SUBCKT dut p<0| p<1| p<2| n<0| n<1| n<2|

  inst_0 n<0| p<0| resistor
  inst_1 n<1| p<1| resistor
  inst_2 n<2| p<2| resistor

.ENDS dut"#,
        "dut vdd vdd vdd 0 0 0 dut",
    ] {
        println!("{:?}", fragment);
        assert!(netlist.contains(fragment));
    }

    let mut buf = Vec::new();
    let netlister = NetlisterInstance::new(NetlistKind::Cells, &schema, &lib, &[], &mut buf);

    netlister.export().unwrap();

    let netlist = std::str::from_utf8(&buf).unwrap();

    println!("{:?}", netlist);
    for fragment in [
        "* netlist_lib",
        r#".SUBCKT dut p<0| p<1| p<2| n<0| n<1| n<2|

  inst_0 n<0| p<0| resistor
  inst_1 n<1| p<1| resistor
  inst_2 n<2| p<2| resistor

.ENDS dut"#,
        r#".SUBCKT tb vss

  dut vdd vdd vdd vss vss vss dut

.ENDS tb"#,
    ] {
        println!("{:?}", fragment);
        assert!(netlist.contains(fragment));
    }
}
