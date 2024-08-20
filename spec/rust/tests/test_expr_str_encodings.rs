// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_str_encodings::*;

#[test]
fn test_expr_str_encodings() -> KResult<()> {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<ExprStrEncodings> = ExprStrEncodings::read_into(&_io, None, None)?;

    assert_eq!(*r.str1_eq()?, true);
    assert_eq!(*r.str2_eq()?, true);
    assert_eq!(*r.str3_eq()?, true);
    assert_eq!(*r.str3_eq_str2()?, true);
    assert_eq!(*r.str4_eq()?, true);
    assert_eq!(*r.str4_gt_str_calc()?, true);
    assert_eq!(*r.str4_gt_str_from_bytes()?, true);
    Ok(())
}