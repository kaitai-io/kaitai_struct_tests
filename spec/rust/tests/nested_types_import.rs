// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::nested_types_import::NestedTypesImport;

#[test]
fn test_nested_types_import() {
    let r = NestedTypesImport::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.a_cc.value_cc, 80);
    assert_eq!(r.a_c_d.value_d, 65);
    assert_eq!(r.b.value_b, 67);
    assert_eq!(r.b.a_cc.value_cc, 75);
    assert_eq!(r.b.a_c_d.value_d, 45);
    assertNull(r.a_cc._parent);
    assertNull(r.a_cc._root);
    assertNull(r.a_c_d._parent);
    assertNull(r.a_c_d._root);
    assertNull(r.b._parent);
    assertNull(r.b._root);
}
