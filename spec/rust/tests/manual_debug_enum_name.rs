#![allow(dead_code)]
extern crate kaitai;
mod formats;
use formats::debug_enum_name::*;

#[test]
fn basic_parse() {
    let r = DebugEnumName::default();
    assert_eq!(*r.test_type().field2(), 0);
}
