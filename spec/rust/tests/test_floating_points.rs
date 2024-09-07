// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::floating_points::*;

#[test]
fn test_floating_points() -> KResult<()> {
    let bytes = fs::read("../../src/floating_points.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<FloatingPoints> = FloatingPoints::read_into(&_io, None, None)?;

    assert_eq!(*r.single_value(), (0.5 as f32));
    assert_eq!(*r.single_value_be(), (0.5 as f32));
    assert_eq!(*r.double_value(), 0.25);
    assert_eq!(*r.double_value_be(), 0.25);
    assert_eq!(*r.approximate_value(), 1.2345);
    assert_eq!(*r.single_value_plus_int()?, 1.5);
    assert_eq!(*r.single_value_plus_float()?, 1.0);
    assert_eq!(*r.double_value_plus_float()?, 0.3);
    Ok(())
}
