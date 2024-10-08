// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_array::*;

#[test]
fn test_expr_array() -> KResult<()> {
    let bytes = fs::read("../../src/expr_array.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<ExprArray> = ExprArray::read_into(&_io, None, None)?;

    assert_eq!(*r.aint_size()?, 4);
    assert_eq!(*r.aint_first()?, 7657765);
    assert_eq!(*r.aint_last()?, 16272640);
    assert_eq!(*r.aint_min()?, 49185);
    assert_eq!(*r.aint_max()?, 1123362332);
    assert_eq!(*r.afloat_size()?, 3);
    assert_eq!(*r.afloat_first()?, -2.6839530254859364E-121);
    assert_eq!(*r.afloat_last()?, -1.1103359815095273E-175);
    assert_eq!(*r.afloat_min()?, -8.754689149998834E+288);
    assert_eq!(*r.afloat_max()?, -1.1103359815095273E-175);
    assert_eq!(*r.astr_size()?, 3);
    assert_eq!(*r.astr_first()?, "foo");
    assert_eq!(*r.astr_last()?, "baz");
    assert_eq!(*r.astr_min()?, "bar");
    assert_eq!(*r.astr_max()?, "foo");
    Ok(())
}
