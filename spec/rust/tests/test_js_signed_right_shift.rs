// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::js_signed_right_shift::JsSignedRightShift;
use std::fs;

#[test]
fn test_js_signed_right_shift() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = JsSignedRightShift::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.should_be_40000000, 1073741824);
    // assert_eq!(r.should_be_a00000, 10485760);
}
