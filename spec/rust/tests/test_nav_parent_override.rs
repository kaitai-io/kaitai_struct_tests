// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::NavParentOverride;

#[test]
fn test_nav_parent_override() {
    if let Ok(r) = NavParentOverride::from_file("src/nav_parent_codes.bin") {
        assert_eq!(r.child_size, 3);
        assert_eq!(r.child_1.data, "\x49\x31\x32");
        assert_eq!(r.mediator_2.child_2.data, "\x33\x42\x62");
    }
}
