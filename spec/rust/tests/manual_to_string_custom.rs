#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::to_string_custom::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);

    let mut r = ToStringCustom::default();
    {
        let res = r.read(&reader, None, Some(KStructUnit::parent_stack()));
        assert!(res.is_ok());
    }

    assert_eq!(r.to_string(), "s1 = foo, s2 = bar");
}
