#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_else_only::*;

#[test]
fn test_switch_else_only() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = SwitchElseOnly::read_into(&reader, None, None);
    let r : Rc<SwitchElseOnly>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }
    assert_eq!(83, *r.opcode());
    let SwitchElseOnly_PrimByte::S1(v) = *r.prim_byte_enum().as_ref().unwrap();
    assert_eq!(102, v);

    if let Some(SwitchElseOnly_Ut::SwitchElseOnly_Data(d)) = (*r.ut()).clone() {
        assert_eq!(vec![0x72, 0x00, 0x49, 0x42], *d.value());
    } else {
        panic!("expected enum SwitchElseOnly_Ut::SwitchElseOnly_Data");
    };
}
