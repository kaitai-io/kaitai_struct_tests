// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nested_types2::*;
use std::fs;

#[test]
fn test_nested_types2() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = NestedTypes2::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    // assert_eq!(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(one)),identifier(typed_at_root)),identifier(value_b)), IntNum(80))
    // assert_eq!(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(one)),identifier(typed_here1)),identifier(value_c)), IntNum(65))
    // assert_eq!(Attribute(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(one)),identifier(typed_here1)),identifier(typed_here)),identifier(value_d)), IntNum(67))
    // assert_eq!(Attribute(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(one)),identifier(typed_here1)),identifier(typed_parent)),identifier(value_cc)), IntNum(75))
    // assert_eq!(Attribute(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(one)),identifier(typed_here1)),identifier(typed_root)),identifier(value_b)), IntNum(45))
    // assert_eq!(Attribute(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(one)),identifier(typed_here2)),identifier(value_cc)), IntNum(49))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(two)),identifier(value_b)), UnaryOp(Minus,IntNum(1)))
}
