// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::process_custom_no_args::*;

#[test]
fn test_process_custom_no_args() -> KResult<()> {
    let bytes = fs::read("../../src/process_rotate.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<ProcessCustomNoArgs> = ProcessCustomNoArgs::read_into(&_io, None, None)?;

    assert_eq!(*r.buf(), vec![0x5fu8, 0x9u8, 0xacu8, 0x8du8, 0x8du8, 0xedu8, 0x5fu8]);
    Ok(())
}
