#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_str::*;

#[test]
fn test_switch_manual_str() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let _io = BytesReader::new(&bytes);
    let res = SwitchManualStr::read_into(&_io, None, None);
    let r : Rc<SwitchManualStr>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }
    assert_eq!(4, r.opcodes().len());

    assert_eq!("S", *r.opcodes()[0].code());
    if let Some(SwitchManualStr_Opcode_Body::SwitchManualStr_Opcode_Strval(s)) = r.opcodes()[0].body().as_ref() {
        assert_eq!("foobar", *s.value());
    } else {
        panic!("expected enum SwitchManualStr_Opcode_Strval");
    }

    assert_eq!("I", *r.opcodes()[1].code());
    if let Some(SwitchManualStr_Opcode_Body::SwitchManualStr_Opcode_Intval(s)) = r.opcodes()[1].body().as_ref() {
        assert_eq!(66, *s.value());
    } else {
        panic!("expected enum SwitchManualStr_Opcode_Intval");
    }

    assert_eq!("I", *r.opcodes()[2].code());
    if let Some(SwitchManualStr_Opcode_Body::SwitchManualStr_Opcode_Intval(s)) = r.opcodes()[2].body().as_ref() {
    assert_eq!(55, *s.value());
    } else {
        panic!("expected enum SwitchManualStr_Opcode_Intval");
    }

    assert_eq!("S", *r.opcodes()[3].code());
    if let Some(SwitchManualStr_Opcode_Body::SwitchManualStr_Opcode_Strval(s)) = r.opcodes()[3].body().as_ref() {
        assert_eq!("", *s.value());
    } else {
        panic!("expected enum SwitchManualStr_Opcode_Strval");
    };
}
