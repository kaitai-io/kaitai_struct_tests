#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::term_bytes::*;

#[test]
fn test_term_bytes() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TermBytes::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.s1(), [0x66u8, 0x6fu8, 0x6fu8].to_vec());
    assert_eq!(*r.s2(), [0x62u8, 0x61u8, 0x72u8].to_vec());
    assert_eq!(*r.s3(), [0x7cu8, 0x62u8, 0x61u8, 0x7au8, 0x40u8].to_vec());
}
