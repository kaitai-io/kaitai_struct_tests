extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::ImportsAbsAbs;

#[test]
fn test_imports_abs_abs() {
    if let Ok(r) = ImportsAbsAbs::from_file("src/fixed_struct.bin") {

        assert_eq!(r.one, 80);
        assert_eq!(r.two.one, 65);
        assert_eq!(r.two.two.one, 67);
    }
}
