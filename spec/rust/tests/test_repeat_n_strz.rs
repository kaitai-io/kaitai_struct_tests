// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::repeat_n_strz::*;
use std::fs;

#[test]
fn test_repeat_n_strz() {
    let data = fs::read("../../src/repeat_n_strz.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = RepeatNStrz::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.qty, 2);
    assert_eq!(r.lines, ["foo", "bar"]);
}
