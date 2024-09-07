// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::bcd_user_type_le::*;

#[test]
fn test_bcd_user_type_le() -> KResult<()> {
    let bytes = fs::read("../../src/bcd_user_type_le.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<BcdUserTypeLe> = BcdUserTypeLe::read_into(&_io, None, None)?;

    assert_eq!(*r.ltr().as_int()?, 12345678);
    assert_eq!(*r.ltr().as_str()?, "12345678");
    assert_eq!(*r.rtl().as_int()?, 87654321);
    assert_eq!(*r.rtl().as_str()?, "87654321");
    assert_eq!(*r.leading_zero_ltr().as_int()?, 123456);
    assert_eq!(*r.leading_zero_ltr().as_str()?, "00123456");
    Ok(())
}
