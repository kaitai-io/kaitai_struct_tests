#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::switch_manual_enum_invalid::*;

#[test]
fn test_switch_manual_enum_invalid() {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<SwitchManualEnumInvalid>> =
        SwitchManualEnumInvalid::read_into(&_io, None, None);
    let r: OptRc<SwitchManualEnumInvalid>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.opcodes().len(), 2);
    let n: i64 = (&*r.opcodes()[0 as usize].code()).into();
    assert_eq!(n, 255);
    assert_eq!(r.opcodes()[0 as usize].body().is_none(), true);
    let n: i64 = (&*r.opcodes()[1 as usize].code()).into();
    assert_eq!(n, 1);
    assert_eq!(r.opcodes()[1 as usize].body().is_none(), true);
}
