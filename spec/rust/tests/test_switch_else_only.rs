#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_else_only::*;

#[test]
fn test_switch_else_only() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchElseOnly::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(83, r.opcode);
    let SwitchElseOnly_PrimByte::S1(v) = r.prim_byte.as_ref().unwrap();
    assert_eq!(102, *v);

    let SwitchElseOnly_Ut::SwitchElseOnly_Data(d) = r.ut.as_ref().unwrap();
    assert_eq!(vec![0x72, 0x00, 0x49, 0x42], d.value);
}
