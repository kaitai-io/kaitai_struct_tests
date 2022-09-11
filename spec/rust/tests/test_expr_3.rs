// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::expr_3::*;
use std::fs;

#[test]
fn test_expr_3() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = Expr3::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.one, 80);
    assert_eq!(r.two, "ACK");
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(three)), Str(@ACK))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(four)), Str(_ACK_))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_eq)), Bool(true))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_ne)), Bool(false))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_lt)), Bool(true))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_gt)), Bool(false))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_le)), Bool(true))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_ge)), Bool(false))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(is_str_lt2)), Bool(true))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(test_not)), Bool(true))
}
