use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::opaque_external_type::*;

#[test]
fn test_opaque_external_type() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<OpaqueExternalType> = OpaqueExternalType::read_into(&_io, None, None)?;

    assert_eq!(*r.one().s1(), "foo");
    assert_eq!(*r.one().s2(), "bar");
    assert_eq!(*r.one().s3(), "|baz@");
    Ok(())
}
