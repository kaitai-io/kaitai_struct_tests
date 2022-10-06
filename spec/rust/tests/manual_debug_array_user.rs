#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::debug_array_user::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = DebugArrayUser::default();
    {
        let res = r.read(&reader, None, Some(KStructUnit::parent_stack()));
        assert!(res.is_ok());
    }

    assert_eq!(*r.one_cat().meow(), 0x50);
    assert_eq!(*r.array_of_cats()[0].meow(), 0x41);
    assert_eq!(*r.array_of_cats()[1].meow(), 0x43);
    assert_eq!(*r.array_of_cats()[2].meow(), 0x4b);
 }
