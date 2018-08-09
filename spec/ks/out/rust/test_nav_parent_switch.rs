// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::NavParentSwitch;

#[test]
fn test_nav_parent_switch() {
    if let Ok(r) = NavParentSwitch::from_file("src/nav_parent_switch.bin") {
        assert_eq!(r.category, 1);
        assert_eq!(r.content.foo, 66);
        assert_eq!(r.content.subelement.bar, 255);
    }
}
