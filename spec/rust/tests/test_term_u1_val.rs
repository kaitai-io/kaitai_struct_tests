#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::term_u1_val::*;

#[test]
fn test_term_u1_val() {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TermU1Val::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.foo(), [0xau8, 0x0u8, 0x53u8, 0x6fu8, 0x6du8, 0x65u8, 0x20u8, 0x41u8, 0x53u8, 0x43u8, 0x49u8, 0x49u8, 0xfu8, 0x0u8].to_vec());
    assert_eq!(r.bar(), "\u{3053}\u{3093}\u{306b}");
}
