#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::expr_1::*;

#[test]
fn test_expr_1() {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = Expr1::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(10, r.len_of_1);

    let res = r.len_of_1_mod(&reader);
    assert_eq!(8, *res.unwrap());

    assert_eq!("Some ASC", *r.str1());

    let res = r.str1_len(&reader);
    assert_eq!(8, *res.unwrap());
}
