use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_enum_invalid::*;


#[test]
fn test_switch_manual_enum_invalid() {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchManualEnumInvalid::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(r.opcodes().len(), 2);
    let n : i64 = r.opcodes()[0 as usize].code().into();
    assert_eq!(n, 255);
    // body is None
    //assert_eq!(r.opcodes()[0 as usize].body(), 0);
    let n : i64 = r.opcodes()[1 as usize].code().into();
    assert_eq!(n, 1);
    // body is None
    //assert_eq!(r.opcodes()[1 as usize].body(), 0);
}
