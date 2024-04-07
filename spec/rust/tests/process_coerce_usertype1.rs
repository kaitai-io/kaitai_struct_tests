// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::process_coerce_usertype1::ProcessCoerceUsertype1;

#[test]
fn test_process_coerce_usertype1() {
    let r = ProcessCoerceUsertype1::from_file("../../src/process_coerce_bytes.bin").expect("file for parsing is not found");

    assert_eq!(r.records[0].flag, 0);
    assert_eq!(r.records[0].buf.value, 1094795585);
    assert_eq!(r.records[1].flag, 1);
    assert_eq!(r.records[1].buf.value, 1111638594);
}
