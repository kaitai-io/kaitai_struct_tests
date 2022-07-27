#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::process_xor_value::*;

#[test]
fn test_process_xor_value() {
    let bytes = fs::read("../../src/process_xor_1.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ProcessXorValue::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(r.key(), 255);
    assert_eq!(*r.buf(), [0x66u8, 0x6fu8, 0x6fu8, 0x20u8, 0x62u8, 0x61u8, 0x72u8].to_vec());
}
