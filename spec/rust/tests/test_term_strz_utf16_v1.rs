// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::term_strz_utf16_v1::*;

#[test]
fn test_term_strz_utf16_v1() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz_utf16.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<TermStrzUtf16V1> = TermStrzUtf16V1::read_into(&_io, None, None)?;

    assert_eq!(*r.s1(), "a\u{200}b");
    assert_eq!(*r.s2(), "c\u{200}d");
    assert_eq!(*r.term(), 0);
    assert_eq!(*r.s3(), "e\u{200}f\u{0}");
    Ok(())
}
