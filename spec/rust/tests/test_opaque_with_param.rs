use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::opaque_with_param::*;

#[test]
fn test_opaque_with_param() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<OpaqueWithParam> = OpaqueWithParam::read_into(&_io, None, None)?;

    assert_eq!(*r.one().buf(), "foo|b");
    assert_eq!(*r.one().trailer(), 0x61);
    Ok(())
}
