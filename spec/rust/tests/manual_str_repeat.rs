#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_repeat::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = StrRepeat::read_into(&_io, None, None);
    let r : Rc<StrRepeat>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!("foo|", r.entries()[0]);
    assert_eq!("bar|", r.entries()[1]);
    assert_eq!("baz@", r.entries()[2]);
}
