// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::enum_of_value_inst::*;

#[test]
fn test_enum_of_value_inst() -> KResult<()> {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<EnumOfValueInst> = EnumOfValueInst::read_into(&_io, None, None)?;

    assert_eq!(*r.pet_1(), EnumOfValueInst_Animal::Cat);
    assert_eq!(*r.pet_2(), EnumOfValueInst_Animal::Chicken);
    assert_eq!(*r.pet_3()?, EnumOfValueInst_Animal::Dog);
    assert_eq!(*r.pet_4()?, EnumOfValueInst_Animal::Dog);
    Ok(())
}
