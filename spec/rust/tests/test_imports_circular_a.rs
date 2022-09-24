use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::imports_circular_a::*;

use formats::imports_circular_b::*;

#[test]
fn test_imports_circular_a() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ImportsCircularA::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.code(), 80);
    assert_eq!(*r.two().as_ref().unwrap().initial(), 65);
    assert_eq!(*r.two().as_ref().unwrap().back_ref().as_ref().unwrap().code(), 67);
    assert_eq!(*r.two().as_ref().unwrap().back_ref().as_ref().unwrap().two().as_ref().unwrap().initial(), 75);
    assert_eq!(*r.two().as_ref().unwrap().back_ref().as_ref().unwrap().two().as_ref().unwrap().back_ref(), None);
}
