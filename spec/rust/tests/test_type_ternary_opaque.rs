use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::type_ternary_opaque::*;

#[test]
fn test_type_ternary_opaque() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<TypeTernaryOpaque> = TypeTernaryOpaque::read_into(&_io, None, None)?;

    assert_eq!(*r.dif()?.s1(), "foo");
    assert_eq!(*r.dif()?.s2(), "bar");
    assert_eq!(*r.dif()?.s3(), "|baz@");
    Ok(())
}
