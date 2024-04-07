// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::nested_type_param::NestedTypeParam;

#[test]
fn test_nested_type_param() {
    let r = NestedTypeParam::from_file("../../src/term_strz.bin").expect("file for parsing is not found");

    assert_eq!(r.main_seq.my_len, 5);
    assert_eq!(r.main_seq.body, "foo|b");
}
