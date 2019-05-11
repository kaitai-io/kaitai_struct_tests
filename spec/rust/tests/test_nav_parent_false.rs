// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::nav_parent_false::NavParentFalse;
use std::fs;

#[test]
fn test_nav_parent_false() {
    let data = fs::read("src/nav_parent_codes.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = NavParentFalse::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.child_size, 3);
    // assert_eq!(r.element_a.foo.code, 73);
    // assert_eq!(r.element_a.foo.more, vec!([0x31, 0x32, 0x33]));
    // assert_eq!(r.element_a.bar.foo.code, 66);
    // assert_eq!(r.element_b.foo.code, 98);
}
