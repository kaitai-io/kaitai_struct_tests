use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::switch_cast::*;

#[test]
#[should_panic(expected = "expected SwitchCast_Opcode_Body::SwitchCast_Strval, got SwitchCast_Intval(")]
fn test_switch_cast() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<SwitchCast> = SwitchCast::read_into(&_io, None, None).unwrap();

    assert_eq!("foobar", *r.first_obj().unwrap().value());
    assert_eq!(0x42, *r.second_val().unwrap());
    let _ = r.err_cast();
}
