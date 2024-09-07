// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::type_int_unary_op::*;

#[test]
fn test_type_int_unary_op() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<TypeIntUnaryOp> = TypeIntUnaryOp::read_into(&_io, None, None)?;

    assert_eq!(*r.value_s2(), 16720);
    assert_eq!(*r.value_s8(), 4706543082108963651);
    assert_eq!(*r.unary_s2()?, -16720);
    assert_eq!(*r.unary_s8()?, -4706543082108963651);
    Ok(())
}
