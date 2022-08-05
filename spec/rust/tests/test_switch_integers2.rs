#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_integers2::*;

#[test]
fn test_switch_integers2() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchIntegers2::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(*r.code(), 1);
    assert_eq!(r.len(), 7);
    assert_eq!(r.ham(), &[0x2u8, 0x40u8, 0x40u8, 0x4u8, 0x37u8, 0x13u8, 0x0u8].to_vec());
    assert_eq!(*r.padding(), 0);
    assert_eq!(*r.len_mod_str(&reader).unwrap(), "13");
}
