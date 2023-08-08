use std::path::PathBuf;

use super::*;

pub(crate) const EXAMPLES_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples");

const VARIABLE: &str = " 0	v(xdut.vdd)	voltage\r\n";

const VARIABLES: &str = r###"Variables:
	0	v(xdut.vdd)	voltage
	1	v(xdut.out)	voltage
	2	i(v.xdut.vdd)	current

"###;

#[test]
fn test_variables() {
    let (_, vars) = variables(VARIABLES.as_bytes()).unwrap();
    println!("{vars:?}");
    assert_eq!(vars.len(), 3);
}

#[test]
fn test_variable() {
    let (_, var) = variable(VARIABLE.as_bytes()).unwrap();
    println!("{var:?}");
    assert_eq!(var.idx, 0);
}

#[test]
fn test_binary_analysis() {
    let path = PathBuf::from(EXAMPLES_PATH).join("rawspice_binary.raw");
    let data = std::fs::read(path).unwrap();
    let (_, analysis) = analysis(&data).unwrap();
    println!("{analysis:?}");

    assert_eq!(analysis.num_variables, 4);
    assert_eq!(analysis.num_points, 301);
    assert_eq!(analysis.variables.len(), 4);

    let data = analysis.data.unwrap_complex();
    assert_eq!(data.len(), 4);
    assert_eq!(data[0].real.len(), 301);
    assert_eq!(data[0].imag.len(), 301);
}

#[test]
fn test_binary_analyses() {
    let path = PathBuf::from(EXAMPLES_PATH).join("rawspice_binary.raw");
    let data = std::fs::read(path).unwrap();
    let (_, mut analyses) = analyses(&data).unwrap();
    println!("{analyses:?}");

    assert_eq!(analyses.len(), 3);

    let analyses2 = analyses.pop().unwrap();
    assert_eq!(analyses2.num_variables, 4);
    assert_eq!(analyses2.num_points, 1008);
    let data2 = analyses2.data.unwrap_real();
    assert_eq!(data2.len(), 4);
    assert_eq!(data2[1].len(), 1008);

    let analyses1 = analyses.pop().unwrap();
    assert_eq!(analyses1.num_variables, 3);
    assert_eq!(analyses1.num_points, 1);
    let data1 = analyses1.data.unwrap_real();
    assert_eq!(data1.len(), 3);
    assert_eq!(data1[1].len(), 1);

    let analyses0 = analyses.pop().unwrap();
    assert_eq!(analyses0.num_variables, 4);
    assert_eq!(analyses0.num_points, 301);
    let data0 = analyses0.data.unwrap_complex();
    assert_eq!(data0.len(), 4);
    assert_eq!(data0[0].real.len(), 301);
    assert_eq!(data0[0].imag.len(), 301);
}

#[test]
fn test_ascii_analysis() {
    let path = PathBuf::from(EXAMPLES_PATH).join("rawspice_ascii.raw");
    let data = std::fs::read(path).unwrap();
    let (_, analysis) = analysis(&data).unwrap();
    println!("{analysis:?}");

    assert_eq!(analysis.num_variables, 4);
    assert_eq!(analysis.num_points, 13);
    assert_eq!(analysis.variables.len(), 4);

    let data = analysis.data.unwrap_complex();
    assert_eq!(data.len(), 4);
    assert_eq!(data[0].real.len(), 13);
    assert_eq!(data[0].imag.len(), 13);
}

#[test]
fn test_ascii_analyses() {
    let path = PathBuf::from(EXAMPLES_PATH).join("rawspice_ascii.raw");
    let data = std::fs::read(path).unwrap();
    let (_, mut analyses) = analyses(&data).unwrap();
    println!("{analyses:?}");

    assert_eq!(analyses.len(), 4);

    let analyses3 = analyses.pop().unwrap();
    assert_eq!(analyses3.num_variables, 4);
    assert_eq!(analyses3.num_points, 59);
    let data3 = analyses3.data.unwrap_real();
    assert_eq!(data3.len(), 4);
    assert_eq!(data3[0].len(), 59);

    let analyses2 = analyses.pop().unwrap();
    assert_eq!(analyses2.num_variables, 3);
    assert_eq!(analyses2.num_points, 1);
    let data2 = analyses2.data.unwrap_real();
    assert_eq!(data2.len(), 3);
    assert_eq!(data2[1].len(), 1);

    let analyses1 = analyses.pop().unwrap();
    assert_eq!(analyses1.num_variables, 4);
    assert_eq!(analyses1.num_points, 6);
    let data1 = analyses1.data.unwrap_real();
    assert_eq!(data1.len(), 4);
    assert_eq!(data1[1].len(), 6);

    let analyses0 = analyses.pop().unwrap();
    assert_eq!(analyses0.num_variables, 4);
    assert_eq!(analyses0.num_points, 13);
    let data0 = analyses0.data.unwrap_complex();
    assert_eq!(data0.len(), 4);
    assert_eq!(data0[1].real.len(), 13);
    assert_eq!(data0[1].imag.len(), 13);
}
