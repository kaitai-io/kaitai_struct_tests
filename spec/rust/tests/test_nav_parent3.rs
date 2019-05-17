// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nav_parent3::*;
use std::fs;

#[test]
fn test_nav_parent3() {
    let data = fs::read("../../src/nav_parent2.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = NavParent3::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.ofs_tags, 8);
    assert_eq!(r.num_tags, 2);
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(0)),identifier(name)), Str(RAHC))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(0)),identifier(ofs)), IntNum(32))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(0)),identifier(num_items)), IntNum(3))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(0)),identifier(tag_content)),identifier(content)), Str(foo))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(1)),identifier(name)), Str(RAHC))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(1)),identifier(ofs)), IntNum(35))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(1)),identifier(num_items)), IntNum(6))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(tags)),IntNum(1)),identifier(tag_content)),identifier(content)), Str(barbaz))
}
