// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nested_types::NestedTypes;
use std::fs;

#[test]
fn test_nested_types() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = NestedTypes::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.one.typed_at_root.value_b, 80);
    // assert_eq!(r.one.typed_here.value_c, 65);
    // assert_eq!(r.two.value_b, 67);
}
