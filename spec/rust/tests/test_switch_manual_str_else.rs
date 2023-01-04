#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_str_else::*;

#[test]
fn test_switch_manual_str_else() {
    let bytes = fs::read("../../src/switch_opcodes2.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = SwitchManualStrElse::read_into(&reader, None, None);
    let r : Rc<SwitchManualStrElse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }
    assert_eq!(4, r.opcodes().len());

    assert_eq!("S", *r.opcodes()[0].code());
    if let Some(SwitchManualStrElse_Opcode_Body::SwitchManualStrElse_Opcode_Strval(s)) = r.opcodes()[0].body().as_ref() {
        assert_eq!("foo", *s.value());
    } else {
        panic!("expected enum SwitchManualStrElse_Opcode_Strval");
    }

    assert_eq!("X", *r.opcodes()[1].code());
    if let Some(SwitchManualStrElse_Opcode_Body::SwitchManualStrElse_Opcode_Noneval(s)) = r.opcodes()[1].body().as_ref() {
        assert_eq!(66, *s.filler());
    } else {
        panic!("expected enum SwitchManualStrElse_Opcode_Noneval");
    }

    assert_eq!("Y", *r.opcodes()[2].code());
    if let Some(SwitchManualStrElse_Opcode_Body::SwitchManualStrElse_Opcode_Noneval(s)) = r.opcodes()[2].body().as_ref() {
    assert_eq!(51966, *s.filler());
    } else {
        panic!("expected enum SwitchManualStrElse_Opcode_Noneval");
    }

    assert_eq!("I", *r.opcodes()[3].code());
    if let Some(SwitchManualStrElse_Opcode_Body::SwitchManualStrElse_Opcode_Intval(s)) = r.opcodes()[3].body().as_ref() {
        assert_eq!(7, *s.value());
    } else {
        panic!("expected enum SwitchManualStrElse_Opcode_Strval");
    };
}
