// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::process_xor4_const::ProcessXor4Const;

#[test]
fn test_process_xor4_const() {
    let r = ProcessXor4Const::from_file("../../src/process_xor_4.bin").expect("file for parsing is not found");

    assert_eq!(r.key, vec!([0xec, 0xbb, 0xa3, 0x14]));
    assert_eq!(r.buf, vec!([0x66, 0x6f, 0x6f, 0x20, 0x62, 0x61, 0x72]));
}
