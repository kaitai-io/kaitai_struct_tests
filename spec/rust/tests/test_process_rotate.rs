#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::process_rotate::*;

#[test]
fn test_process_rotate() {
    let bytes = fs::read("../../src/process_rotate.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ProcessRotate::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.buf1(), [0x48u8, 0x65u8, 0x6cu8, 0x6cu8, 0x6fu8].to_vec());
    assert_eq!(*r.buf2(), [0x57u8, 0x6fu8, 0x72u8, 0x6cu8, 0x64u8].to_vec());
    assert_eq!(*r.buf3(), [0x54u8, 0x68u8, 0x65u8, 0x72u8, 0x65u8].to_vec());
}
