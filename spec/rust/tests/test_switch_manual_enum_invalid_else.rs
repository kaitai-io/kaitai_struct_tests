#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_enum_invalid_else::*;


#[test]
fn test_switch_manual_enum_invalid_else() {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<SwitchManualEnumInvalidElse>> = SwitchManualEnumInvalidElse::read_into(&_io, None, None);
    let r : OptRc<SwitchManualEnumInvalidElse>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.opcodes().len(), 2);
    let n : i64 = (&*r.opcodes()[0 as usize].code()).into();
    assert_eq!(n, 255);
    assert_eq!(*Into::<OptRc<SwitchManualEnumInvalidElse_Opcode_Defval>>::into(&*r.opcodes()[0 as usize].body().as_ref().unwrap()).value().unwrap(), 123);
    let n : i64 = (&*r.opcodes()[1 as usize].code()).into();
    assert_eq!(n, 1);
    assert_eq!(*Into::<OptRc<SwitchManualEnumInvalidElse_Opcode_Defval>>::into(&*r.opcodes()[1 as usize].body().as_ref().unwrap()).value().unwrap(), 123);
}
