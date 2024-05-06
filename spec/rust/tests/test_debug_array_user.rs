#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
use rust::formats::debug_array_user::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = DebugArrayUser::read_into(&_io, None, None);
    let r : OptRc<DebugArrayUser>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one_cat().meow(), 0x50);
    assert_eq!(*r.array_of_cats()[0].meow(), 0x41);
    assert_eq!(*r.array_of_cats()[1].meow(), 0x43);
    assert_eq!(*r.array_of_cats()[2].meow(), 0x4b);
 }
