use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::opaque_external_type_02_parent::*;

#[test]
fn test_opaque_external_type_02_parent() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<OpaqueExternalType02Parent> = OpaqueExternalType02Parent::read_into(&_io, None, None)?;

    assert_eq!(*r.parent().child().s1(), "foo");
    assert_eq!(*r.parent().child().s2(), "bar");
    assert_eq!(*r.parent().child().s3().s3(), "|baz@");
    Ok(())
}
