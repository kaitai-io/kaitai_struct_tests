#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_int::*;

#[test]
fn test_switch_manual_int() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchManualInt::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(83, *r.opcodes()[0].code());
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Strval(s) =  r.opcodes()[0].body() {
        assert_eq!("foobar", s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(73, *r.opcodes()[1].code());
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Intval(s) =  r.opcodes()[1].body() {
        assert_eq!(66, *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(73, *r.opcodes()[2].code());
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Intval(s) =  r.opcodes()[2].body() {
    assert_eq!(55, *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }

    assert_eq!(83, *r.opcodes()[3].code());
    if let SwitchManualInt_Opcode_Body::SwitchManualInt_Opcode_Strval(s) =  r.opcodes()[3].body() {
        assert_eq!("", *s.value());
    } else {
        panic!("expected enum SwitchManualInt_Opcode_Body");
    }
}
