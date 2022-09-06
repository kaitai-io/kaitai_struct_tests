use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::enum_for_unknown_id::*;


#[test]
fn test_enum_for_unknown_id() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = EnumForUnknownId::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        println!("expected err: {:?}, exception: UnknownVariant", err);
    } else {
        panic!("no expected exception: UnknownVariant");
    }

    //assert_eq!(r.one().clone().to_owned() as u64, 80);
}
