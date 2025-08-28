use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::str_literals::*;

#[test]
fn test_str_literals() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<StrLiterals> = StrLiterals::read_into(&_io, None, None)?;

    assert_eq!(*r.complex_str()?, "\u{0}\u{1}\u{2}\u{7}\u{8}\u{a}\u{d}\u{9}\u{b}\u{c}\u{1b}\u{3d}\u{7}\u{a}\u{24}\u{263b}");
    assert_eq!(*r.double_quotes()?, "\u{22}\u{22}\u{22}");
    assert_eq!(*r.backslashes()?, "\u{5c}\u{5c}\u{5c}");
    assert_eq!(*r.octal_eatup()?, "\u{0}\u{32}\u{32}");
    assert_eq!(*r.octal_eatup2()?, "\u{2}\u{32}");
    Ok(())
}
