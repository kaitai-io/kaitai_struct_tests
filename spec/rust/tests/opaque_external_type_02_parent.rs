extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::opaque_external_type_02_parent::OpaqueExternalType02Parent;

#[test]
fn test_opaque_external_type_02_parent() {
    let r = OpaqueExternalType02Parent::from_file("../../src/term_strz.bin").expect("file for parsing is not found");

    assert_eq!(r.parent.child.s1, "foo");
    assert_eq!(r.parent.child.s2, "bar");
    assert_eq!(r.parent.child.s3.s3, "|baz@");
}
