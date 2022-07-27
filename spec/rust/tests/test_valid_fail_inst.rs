#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::valid_fail_inst::*;

#[test]
fn test_valid_fail_inst() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ValidFailInst::default();

    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {
        println!("expected err: {:?}, exception: ValidationNotEqualError(Int1Type(false))", err);
    } else {
        panic!("no expected exception: ValidationNotEqualError(Int1Type(false))");
    }
}
