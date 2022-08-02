#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::process_to_user::*;

#[test]
fn test_process_to_user() {
    let bytes = fs::read("../../src/process_rotate.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ProcessToUser::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(r.buf1().str(), "Hello");
}
