use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::optional_id::*;

#[test]
fn test_optional_id() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<OptionalId> = OptionalId::read_into(&_io, None, None)?;

    assert_eq!(*r.unnamed0(), 80);
    assert_eq!(*r.unnamed1(), 65);
    assert_eq!(*r.unnamed2(), vec![0x43, 0x4B, 0x2D, 0x31, 0xFF]);
    Ok(())
}
