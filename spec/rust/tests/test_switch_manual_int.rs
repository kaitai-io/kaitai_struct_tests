#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_int::*;

#[test]
fn test_switch_manual_int() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = SwitchManualInt::read_into(&reader, None, None);
    let r : Rc<SwitchManualInt>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }
    assert_eq!(83, *r.opcodes()[0].code());
    if let Some(SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Strval(s)) = r.opcodes()[0].body().as_ref() {
        assert_eq!("foobar", *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(73, *r.opcodes()[1].code());
    if let Some(SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Intval(s)) = r.opcodes()[1].body().as_ref() {
        assert_eq!(66, *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(73, *r.opcodes()[2].code());
    if let Some(SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Intval(s)) = r.opcodes()[2].body().as_ref() {
    assert_eq!(55, *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(83, *r.opcodes()[3].code());
    if let Some(SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Strval(s)) = r.opcodes()[3].body().as_ref() {
        assert_eq!("", *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    };
}
