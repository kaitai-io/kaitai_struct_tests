// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::enum_deep::*;
use std::fs;

#[test]
fn test_enum_deep() {
    let data = fs::read("../../src/enum_0.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = EnumDeep::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(pet_1)), EnumByLabel(identifier(animal),identifier(cat),typeId(false,ArrayBuffer(enum_deep, container1),false)))
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(pet_2)), EnumByLabel(identifier(animal),identifier(hare),typeId(false,ArrayBuffer(enum_deep, container1, container2),false)))
}
