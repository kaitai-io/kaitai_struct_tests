#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::type_int_unary_op::*;

#[test]
fn test_type_int_unary_op() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TypeIntUnaryOp::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.value_s2(), 16720);
    assert_eq!(*r.value_s8(), 4706543082108963651);
    assert_eq!(*r.unary_s2(&reader).unwrap(), -16720);
    assert_eq!(*r.unary_s8(&reader).unwrap(), -4706543082108963651);
}
