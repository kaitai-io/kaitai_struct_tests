use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::to_string_custom::*;

#[test]
fn test_to_string_custom() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<ToStringCustom> = ToStringCustom::read_into(&_io, None, None)?;

    assert_eq!(r.to_string(), "s1 = foo, s2 = bar");
    Ok(())
}
