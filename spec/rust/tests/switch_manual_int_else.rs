// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::switch_manual_int_else::SwitchManualIntElse;

#[test]
fn test_switch_manual_int_else() {
    let r = SwitchManualIntElse::from_file("../../src/switch_opcodes2.bin").expect("file for parsing is not found");

    assert_eq!(r.opcodes.len(), 4);
    assert_eq!(r.opcodes[0].code, 83);
    assert_eq!(r.opcodes[0].body.value, "foo");
    assert_eq!(r.opcodes[1].code, 88);
    assert_eq!(r.opcodes[1].body.filler, 66);
    assert_eq!(r.opcodes[2].code, 89);
    assert_eq!(r.opcodes[2].body.filler, 51966);
    assert_eq!(r.opcodes[3].code, 73);
    assert_eq!(r.opcodes[3].body.value, 7);
}
