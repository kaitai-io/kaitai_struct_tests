#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_literals::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::new(&bytes);
    let res = StrLiterals::read_into(&_io, None, None);
    let r : Rc<StrLiterals>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    let backslashes = r.backslashes(&_io).unwrap();
    assert_eq!("\u{005c}\u{005c}\u{005c}", *backslashes);
    
    let octal_eatup = r.octal_eatup(&_io).unwrap();
    assert_eq!("\u{0}\u{0032}\u{0032}", *octal_eatup);

    let octal_eatup2 = r.octal_eatup2(&_io).unwrap();
    assert_eq!("\u{2}\u{32}", *octal_eatup2);

    let double_quotes = r.double_quotes(&_io).unwrap();
    assert_eq!("\u{22}\u{22}\u{22}", *double_quotes);

    let complex_str = r.complex_str(&_io).unwrap();
    assert_eq!("\u{0}\u{1}\u{2}\u{7}\u{8}\u{0a}\u{0d}\u{09}\u{0b}\u{c}\u{1b}\u{3d}\u{7}\u{a}\u{24}\u{263b}", *complex_str);
}