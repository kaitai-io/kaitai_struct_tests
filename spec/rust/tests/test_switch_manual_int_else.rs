#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_int_else::*;

#[test]
fn test_switch_manual_int_else() {
    let bytes = fs::read("../../src/switch_opcodes2.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchManualIntElse::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
    assert_eq!(4, r.opcodes().len());

    assert_eq!(83, r.opcodes()[0].code());
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Strval(s) =  r.opcodes[0].body() {
        assert_eq!("foo", s.value);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Strval");
    }

    assert_eq!(88, r.opcodes()[1].code());
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Noneval(s) =  r.opcodes[1].body() {
        assert_eq!(66, s.filler);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Noneval");
    }

    assert_eq!(89, r.opcodes()[2].code());
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Noneval(s) =  r.opcodes[2].body() {
    assert_eq!(51966, s.filler);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Noneval");
    }

    assert_eq!(73, r.opcodes()[3].code());
    if let SwitchManualIntElse_Opcode_Body::SwitchManualIntElse_Opcode_Intval(s) =  r.opcodes[3].body() {
        assert_eq!(7, s.value);
    } else {
        panic!("expected enum SwitchManualIntElse_Opcode_Intval");
    }
}
