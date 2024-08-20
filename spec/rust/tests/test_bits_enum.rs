// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::bits_enum::*;

#[test]
fn test_bits_enum() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<BitsEnum> = BitsEnum::read_into(&_io, None, None)?;

    assert_eq!(*r.one(), BitsEnum_Animal::Platypus);
    assert_eq!(*r.two(), BitsEnum_Animal::Horse);
    assert_eq!(*r.three(), BitsEnum_Animal::Cat);
    Ok(())
}