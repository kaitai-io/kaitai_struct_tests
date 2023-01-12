#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::to_string_custom::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::new(&bytes);
    let res = ToStringCustom::read_into(&_io, None, None);
    let r : Rc<ToStringCustom>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.to_string(), "s1 = foo, s2 = bar");
}
