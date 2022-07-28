use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::valid_switch::*;

#[test]
fn test_valid_switch() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ValidSwitch::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {

        panic!("{:?}", err);
    }
}
