use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::debug_0::*;

#[test]
fn test_debug_0() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<Debug0> = Debug0::read_into(&_io, None, None)?;
    assert_eq!(*r.one(), 80);
    assert_eq!(r.array_of_ints().len(), 3);
    assert_eq!(r.array_of_ints()[0 as usize], 65);
    assert_eq!(r.array_of_ints()[1 as usize], 67);
    assert_eq!(r.array_of_ints()[2 as usize], 75);
    assert_eq!(*r.unnamed2(), 45);
    Ok(())
}
