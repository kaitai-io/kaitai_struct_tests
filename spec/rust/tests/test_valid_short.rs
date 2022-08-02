use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::valid_short::*;

#[test]
fn test_valid_short() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ValidShort::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
}
