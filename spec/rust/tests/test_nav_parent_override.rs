// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nav_parent_override::NavParentOverride;
use std::fs;

#[test]
fn test_nav_parent_override() {
    let data = fs::read("../../src/nav_parent_codes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = NavParentOverride::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.child_size, 3);
    // assert_eq!(r.child_1.data, vec!([0x49, 0x31, 0x32]));
    // assert_eq!(r.mediator_2.child_2.data, vec!([0x33, 0x42, 0x62]));
}
