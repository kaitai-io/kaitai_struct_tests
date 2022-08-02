#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_pad_term::*;

#[test]
fn test_str_pad_term() {
    let bytes = fs::read("../../src/str_pad_term.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = StrPadTerm::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(r.str_pad(), "str1");
    assert_eq!(r.str_term(), "str2foo");
    assert_eq!(r.str_term_and_pad(), "str+++3bar+++");
    assert_eq!(r.str_term_include(), "str4baz@");
}
