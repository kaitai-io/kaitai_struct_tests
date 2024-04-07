// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::process_repeat_usertype::ProcessRepeatUsertype;

#[test]
fn test_process_repeat_usertype() {
    let r = ProcessRepeatUsertype::from_file("../../src/process_xor_4.bin").expect("file for parsing is not found");

    assert_eq!(r.blocks[0].a, -1975704206);
    assert_eq!(r.blocks[0].b, 20);
    assert_eq!(r.blocks[1].a, 279597642);
    assert_eq!(r.blocks[1].b, 68);
}
