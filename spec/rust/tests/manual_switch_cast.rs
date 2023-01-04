#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_cast::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let reader = BytesReader::new(&bytes);

    let res = SwitchCast::read_into(&reader, None, None);
    let r : Rc<SwitchCast>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(4, r.opcodes().len());

    assert_eq!(0x53, *r.opcodes()[0].code());
    if let Some(SwitchCast_Opcode_Body::SwitchCast_Strval(s)) = r.opcodes()[0].body().as_ref() {
        assert_eq!("foobar", *s.value());
    } else {
        panic!("expected enum SwitchCast_Opcode_Strval");
    }

    assert_eq!(0x49, *r.opcodes()[1].code());
    if let Some(SwitchCast_Opcode_Body::SwitchCast_Intval(s)) = r.opcodes()[1].body().as_ref() {
        assert_eq!(66, *s.value());
    } else {
        panic!("expected enum SwitchCast_Opcode_Intval");
    }

    assert_eq!(0x49, *r.opcodes()[2].code());
    if let Some(SwitchCast_Opcode_Body::SwitchCast_Intval(s)) = r.opcodes()[2].body().as_ref() {
        assert_eq!(55, *s.value());
    } else {
        panic!("expected enum SwitchCast_Opcode_Intval");
    }

    assert_eq!(0x53, *r.opcodes()[3].code());
    if let Some(SwitchCast_Opcode_Body::SwitchCast_Strval(s)) = r.opcodes()[3].body().as_ref() {
        assert_eq!("", *s.value());
    } else {
        panic!("expected enum SwitchCast_Opcode_Strval");
    };
}