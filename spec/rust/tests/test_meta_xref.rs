// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::meta_xref::*;

#[test]
fn test_meta_xref() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<MetaXref> = MetaXref::read_into(&_io, None, None)?;

    Ok(())
}
