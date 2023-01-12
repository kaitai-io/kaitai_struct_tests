use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_enum_invalid_else::*;


#[test]
fn test_switch_manual_enum_invalid_else() {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let _io = BytesReader::new(&bytes);
    let res = SwitchManualEnumInvalidElse::read_into(&_io, None, None);
    let r : Rc<SwitchManualEnumInvalidElse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.opcodes().len(), 2);
    let n : i64 = (&*r.opcodes()[0 as usize].code()).into();
    assert_eq!(n, 255);
    if let Some(SwitchManualEnumInvalidElse_Opcode_Body::SwitchManualEnumInvalidElse_Opcode_Defval(s)) = r.opcodes()[0].body().as_ref() {
        assert_eq!(*s.value(&_io).unwrap(), 123);
    } else {
        panic!("expected enum SwitchManualEnumInvalidElse_Opcode_Defval");
    };
    let n : i64 = (&*r.opcodes()[1 as usize].code()).into();
    assert_eq!(n, 1);
    if let Some(SwitchManualEnumInvalidElse_Opcode_Body::SwitchManualEnumInvalidElse_Opcode_Defval(s)) = r.opcodes()[1].body().as_ref() {
        assert_eq!(*s.value(&_io).unwrap(), 123);
    } else {
        panic!("expected enum SwitchManualEnumInvalidElse_Opcode_Defval");
    };
}
