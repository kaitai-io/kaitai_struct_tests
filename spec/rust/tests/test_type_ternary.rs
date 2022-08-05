#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::type_ternary::*;

#[test]
fn test_type_ternary() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TypeTernary::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.dif(&reader).unwrap().value(), 101);
}
