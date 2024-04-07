// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::repeat_n_struct::RepeatNStruct;

#[test]
fn test_repeat_n_struct() {
    let r = RepeatNStruct::from_file("../../src/repeat_n_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.chunks.len(), 2);
    assert_eq!(r.chunks[0].offset, 16);
    assert_eq!(r.chunks[0].len, 8312);
    assert_eq!(r.chunks[1].offset, 8328);
    assert_eq!(r.chunks[1].len, 15);
}
