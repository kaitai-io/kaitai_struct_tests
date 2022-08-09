// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::enum_deep::*;

#[test]
fn test_enum_deep() {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = EnumDeep::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(r.pet_1(), &EnumDeep_Container1_Animal::Cat);
    assert_eq!(r.pet_2(), &EnumDeep_Container1_Container2_Animal::Hare);
}