#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_integers::*;

#[test]
fn test_switch_integers() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchIntegers::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(r.opcodes().len(), 4);
    assert_eq!(*r.opcodes()[0 as usize].code(), 1);
    assert_eq!(r.opcodes()[0 as usize].body(), 7);
    assert_eq!(*r.opcodes()[1 as usize].code(), 2);
    assert_eq!(r.opcodes()[1 as usize].body(), 16448);
    assert_eq!(*r.opcodes()[2 as usize].code(), 4);
    assert_eq!(r.opcodes()[2 as usize].body(), 4919);
    assert_eq!(*r.opcodes()[3 as usize].code(), 8);
    assert_eq!(r.opcodes()[3 as usize].body(), 4919);
}
