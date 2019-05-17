// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nav_parent_override::*;
use std::fs;

#[test]
fn test_nav_parent_override() {
    let data = fs::read("../../src/nav_parent_codes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = NavParentOverride::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.child_size, 3);
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(child_1)),identifier(data)), List(ArrayBuffer(IntNum(73), IntNum(49), IntNum(50))))
    // assert_eq!(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(mediator_2)),identifier(child_2)),identifier(data)), List(ArrayBuffer(IntNum(51), IntNum(66), IntNum(98))))
}
