// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::repeat_eos_u4::*;

#[test]
fn test_repeat_eos_u4() -> KResult<()> {
    let bytes = fs::read("../../src/repeat_eos_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<RepeatEosU4> = RepeatEosU4::read_into(&_io, None, None)?;

    assert_eq!(*r.numbers(), vec![0, 66, 66, 2069]);
    Ok(())
}
