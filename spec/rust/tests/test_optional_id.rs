#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::optional_id::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::new(&bytes);
    let res = OptionalId::read_into(&_io, None, None);
    let r : Rc<OptionalId>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.unnamed0(), 80);
    assert_eq!(*r.unnamed1(), 65);
    assert_eq!(*r.unnamed2(), vec![0x43, 0x4B, 0x2D, 0x31, 0xFF]);
}
