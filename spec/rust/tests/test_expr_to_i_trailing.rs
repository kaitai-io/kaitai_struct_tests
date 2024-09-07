// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_to_i_trailing::*;

#[test]
fn test_expr_to_i_trailing() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<ExprToITrailing> = ExprToITrailing::read_into(&_io, None, None)?;

    let err = r.to_i_r10().expect_err("expected Err representing ConversionError, but got Ok");
    assert_eq!(err, KError::CastError);
    assert_eq!(*r.to_i_r16()?, 152517308);
    let err = r.to_i_garbage().expect_err("expected Err representing ConversionError, but got Ok");
    assert_eq!(err, KError::CastError);
    Ok(())
}
