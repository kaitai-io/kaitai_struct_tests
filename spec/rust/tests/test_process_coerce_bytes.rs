#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::process_coerce_bytes::*;

#[test]
fn test_process_coerce_bytes() {
    let bytes = fs::read("../../src/process_coerce_bytes.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ProcessCoerceBytes::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(0, r.records[0].flag());

    let mut i = r.records.iter_mut();

    let buf : Vec<u8> = vec![0x41, 0x41, 0x41, 0x41];
    let x = i.next().unwrap();
    assert_eq!(&buf, x.buf(&reader).unwrap());

    let buf : Vec<u8> = vec![0x42, 0x42, 0x42, 0x42];
    let x = i.next().unwrap();
    assert_eq!(&buf, x.buf(&reader).unwrap());
}
