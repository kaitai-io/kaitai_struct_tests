use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_enum_invalid_else::*;


#[test]
fn test_switch_manual_enum_invalid_else() {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = SwitchManualEnumInvalidElse::read_into(&reader, None, None);
    let r : Rc<SwitchManualEnumInvalidElse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.opcodes().len(), 2);
    let n : i64 = r.opcodes()[0 as usize].code().into();
    assert_eq!(n, 255);
    if let SwitchManualEnumInvalidElse_Opcode_Body::SwitchManualEnumInvalidElse_Opcode_Defval(s) =  r.opcodes()[0].body() {
        assert_eq!(*s.value(&reader).unwrap(), 123);
    } else {
        panic!("expected enum SwitchManualEnumInvalidElse_Opcode_Defval");
    }
    let n : i64 = r.opcodes()[1 as usize].code().into();
    assert_eq!(n, 1);
    if let SwitchManualEnumInvalidElse_Opcode_Body::SwitchManualEnumInvalidElse_Opcode_Defval(s) =  r.opcodes()[1].body() {
        assert_eq!(*s.value(&reader).unwrap(), 123);
    } else {
        panic!("expected enum SwitchManualEnumInvalidElse_Opcode_Defval");
    }
}
