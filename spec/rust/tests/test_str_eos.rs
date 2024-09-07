// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::str_eos::*;

#[test]
fn test_str_eos() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<StrEos> = StrEos::read_into(&_io, None, None)?;

    assert_eq!(*r.str(), "foo|bar|baz@");
    Ok(())
}
