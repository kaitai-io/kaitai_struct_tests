// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::nested_types2::NestedTypes2;

#[test]
fn test_nested_types2() {
    let r = NestedTypes2::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.one.typed_at_root.value_b, 80);
    assert_eq!(r.one.typed_here1.value_c, 65);
    assert_eq!(r.one.typed_here1.typed_here.value_d, 67);
    assert_eq!(r.one.typed_here1.typed_parent.value_cc, 75);
    assert_eq!(r.one.typed_here1.typed_root.value_b, 45);
    assert_eq!(r.one.typed_here2.value_cc, 49);
    assert_eq!(r.two.value_b, -1);
}
