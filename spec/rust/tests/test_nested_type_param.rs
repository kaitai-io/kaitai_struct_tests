// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::nested_type_param::*;

#[test]
fn test_nested_type_param() -> KResult<()> {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<NestedTypeParam> = NestedTypeParam::read_into(&_io, None, None)?;

    assert_eq!(*r.main_seq().my_len(), 5);
    assert_eq!(*r.main_seq().body(), "foo|b");
    Ok(())
}
