#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::expr_0::*;

#[test]
fn test_expr_0() {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = Expr0::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.must_be_f7(&reader).unwrap(), 247);
    assert_eq!(r.must_be_abc123(&reader).unwrap(), "abc123");
}
