#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::read_bytes::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = ReadBytes::read_into(&_io, None, None);
    let r : Rc<ReadBytes>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(1, *r.len());
    assert_eq!(vec![7], *r.val());
    assert_eq!(vec![0x2, 0x40], *r.padding());
}
