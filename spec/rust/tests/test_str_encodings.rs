#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_encodings::*;

#[test]
fn test_str_encodings() {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = StrEncodings::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(r.str1(), "Some ASCII");
    assert_eq!(r.str2(), "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}");
    assert_eq!(r.str3(), "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}");
    assert_eq!(r.str4(), "\u{2591}\u{2592}\u{2593}");
}
