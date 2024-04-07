// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::switch_bytearray::SwitchBytearray;

#[test]
fn test_switch_bytearray() {
    let r = SwitchBytearray::from_file("../../src/switch_opcodes.bin").expect("file for parsing is not found");

    assert_eq!(r.opcodes.len(), 4);
    assert_eq!(r.opcodes[0].code, vec!([0x53]));
    assert_eq!(r.opcodes[0].body.value, "foobar");
    assert_eq!(r.opcodes[1].code, vec!([0x49]));
    assert_eq!(r.opcodes[1].body.value, 66);
    assert_eq!(r.opcodes[2].code, vec!([0x49]));
    assert_eq!(r.opcodes[2].body.value, 55);
    assert_eq!(r.opcodes[3].code, vec!([0x53]));
    assert_eq!(r.opcodes[3].body.value, "");
}
