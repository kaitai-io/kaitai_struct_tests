// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::imports_circular_a::ImportsCircularA;

#[test]
fn test_imports_circular_a() {
    let r = ImportsCircularA::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.code, 80);
    assert_eq!(r.two.initial, 65);
    assert_eq!(r.two.back_ref.code, 67);
    assert_eq!(r.two.back_ref.two.initial, 75);
    assertNull(r.two.back_ref.two.back_ref);
}
