// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::enum_of_value_inst::EnumOfValueInst;
use std::fs;

#[test]
fn test_enum_of_value_inst() {
    let data = fs::read("../../src/enum_0.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = EnumOfValueInst::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.pet_1, animal::CAT);
    // assert_eq!(r.pet_2, animal::CHICKEN);
    // assert_eq!(r.pet_3, animal::DOG);
    // assert_eq!(r.pet_4, animal::DOG);
}
