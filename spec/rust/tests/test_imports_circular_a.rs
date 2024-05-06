#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::imports_circular_a::*;
use rust::formats::imports_circular_b::*;

#[test]
fn test_imports_circular_a() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ImportsCircularA>> = ImportsCircularA::read_into(&_io, None, None);
    let r: OptRc<ImportsCircularA>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.code(), 80);
    assert_eq!(*r.two().initial(), 65);
    assert_eq!(*r.two().back_ref().code(), 67);
    assert_eq!(*r.two().back_ref().two().initial(), 75);
    assert_eq!(r.two().back_ref().two().back_ref().is_none(), true);
}
