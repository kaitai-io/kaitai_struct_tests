#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::process_xor4_value::*;

#[test]
fn test_process_xor4_value() {
    let bytes = fs::read("../../src/process_xor_4.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ProcessXor4Value::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.key(), [0xecu8, 0xbbu8, 0xa3u8, 0x14u8].to_vec());
    assert_eq!(*r.buf(), [0x66u8, 0x6fu8, 0x6fu8, 0x20u8, 0x62u8, 0x61u8, 0x72u8].to_vec());
}
