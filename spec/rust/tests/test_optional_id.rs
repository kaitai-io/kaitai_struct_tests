#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::optional_id::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);

    let mut r = OptionalId::default();
    {
        let res = r.read(&reader, None, Some(KStructUnit::parent_stack()));
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(*r.unnamed0(), 80);
    assert_eq!(*r.unnamed1(), 65);
    assert_eq!(*r.unnamed2(), vec![0x43, 0x4B, 0x2D, 0x31, 0xFF]);
}
