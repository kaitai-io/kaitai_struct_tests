#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::read_bytes::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let reader = BytesReader::new(&bytes);

    let mut r = ReadBytes::default();
    {
        let res = r.read(&reader, None, KStructUnit::parent_stack());
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(1, r.len());
    assert_eq!(vec![7], *r.val());
    assert_eq!(vec![0x2, 0x40], *r.padding());
}
