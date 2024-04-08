extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::NestedTypesImport;

#[test]
fn test_nested_types_import() {
    if let Ok(r) = NestedTypesImport::from_file("src/fixed_struct.bin") {

        assert_eq!(r.a_cc.value_cc, 80);
        assert_eq!(r.a_c_d.value_d, 65);
        assert_eq!(r.b.value_b, 67);
        assert_eq!(r.b.a_cc.value_cc, 75);
        assert_eq!(r.b.a_c_d.value_d, 45);
    }
}
