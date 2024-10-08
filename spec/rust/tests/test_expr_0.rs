// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_0::*;

#[test]
fn test_expr_0() -> KResult<()> {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<Expr0> = Expr0::read_into(&_io, None, None)?;

    assert_eq!(*r.must_be_f7()?, 247);
    assert_eq!(*r.must_be_abc123()?, "abc123");
    Ok(())
}
