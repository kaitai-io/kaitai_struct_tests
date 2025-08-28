use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::nested_types_import::*;

#[test]
fn test_nested_types_import() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<NestedTypesImport> = NestedTypesImport::read_into(&_io, None, None)?;

    assert_eq!(*r.a_cc().value_cc(), 80);
    assert_eq!(*r.a_c_d().value_d(), 65);
    assert_eq!(*r.b().value_b(), 67);
    assert_eq!(*r.b().a_cc().value_cc(), 75);
    assert_eq!(*r.b().a_c_d().value_d(), 45);
    assert!(r.a_cc()._parent.get_value().borrow().upgrade().is_none());
    assert!(r.a_cc()._root.get_value().borrow().upgrade().is_none());
    assert!(r.a_c_d()._parent.get_value().borrow().upgrade().is_none());
    assert!(r.a_c_d()._root.get_value().borrow().upgrade().is_none());
    assert!(r.b()._parent.get_value().borrow().upgrade().is_none());
    assert!(r.b()._root.get_value().borrow().upgrade().is_none());
    Ok(())
}
