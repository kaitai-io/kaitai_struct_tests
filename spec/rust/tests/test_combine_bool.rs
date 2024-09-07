// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::combine_bool::*;

#[test]
fn test_combine_bool() -> KResult<()> {
    let bytes = fs::read("../../src/enum_negative.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<CombineBool> = CombineBool::read_into(&_io, None, None)?;

    assert_eq!(*r.bool_bit(), true);
    assert_eq!(*r.bool_calc_bit()?, false);
    Ok(())
}
