use std::fs;

extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::switch_manual_enum_invalid::*;


#[test]
fn test_switch_manual_enum_invalid() {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = SwitchManualEnumInvalid::read_into(&_io, None, None);
    let r : OptRc<SwitchManualEnumInvalid>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.opcodes().len(), 2);
    let n : i64 = (&*r.opcodes()[0 as usize].code()).into();
    assert_eq!(n, 255);
    // body is None
    //assert_eq!(r.opcodes()[0 as usize].body(), 0);
    let n : i64 = (&*r.opcodes()[1 as usize].code()).into();
    assert_eq!(n, 1);
    // body is None
    //assert_eq!(r.opcodes()[1 as usize].body(), 0);
}
