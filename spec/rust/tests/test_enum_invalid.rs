use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::enum_invalid::*;


#[test]
fn test_enum_invalid() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = EnumInvalid::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        println!("expected err: {:?}, exception: UnknownVariant", err);
    } else {
        panic!("no expected exception: UnknownVariant");
    }

    // assert_eq!(*r.pet_1(), EnumInvalid_Animal::Dog);
    // assert_eq!(r.pet_2().clone().to_owned() as u64, 111);
}
