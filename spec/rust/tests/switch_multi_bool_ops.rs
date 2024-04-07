// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::switch_multi_bool_ops::SwitchMultiBoolOps;

#[test]
fn test_switch_multi_bool_ops() {
    let r = SwitchMultiBoolOps::from_file("../../src/switch_integers.bin").expect("file for parsing is not found");

    assert_eq!(r.opcodes.len(), 4);
    assert_eq!(r.opcodes[0].code, 1);
    assert_eq!(r.opcodes[0].body, 7);
    assert_eq!(r.opcodes[1].code, 2);
    assert_eq!(r.opcodes[1].body, 16448);
    assert_eq!(r.opcodes[2].code, 4);
    assert_eq!(r.opcodes[2].body, 4919);
    assert_eq!(r.opcodes[3].code, 8);
    assert_eq!(r.opcodes[3].body, 4919);
}
