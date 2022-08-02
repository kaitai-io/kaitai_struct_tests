#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_repeat::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = StrRepeat::default();
    {
        let res = r.read(&reader, None, Some(KStructUnit::parent_stack()));
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!("foo|", r.entries()[0]);
    assert_eq!("bar|", r.entries()[1]);
    assert_eq!("baz@", r.entries()[2]);
}
