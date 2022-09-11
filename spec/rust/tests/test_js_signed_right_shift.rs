// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::js_signed_right_shift::*;
use std::fs;

#[test]
fn test_js_signed_right_shift() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = JsSignedRightShift::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(should_be_40000000)), IntNum(1073741824))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(should_be_a00000)), IntNum(10485760))
}
