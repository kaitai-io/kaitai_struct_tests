#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_literals2::*;

#[test]
fn test_str_literals2() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = StrLiterals2::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    let dollar1 = r.dollar1(&reader).unwrap();
    assert_eq!("$foo", *dollar1);
    
    let dollar2 = r.dollar2(&reader).unwrap();
    assert_eq!("${foo}", *dollar2);

    let hash = r.hash(&reader).unwrap();
    assert_eq!("#{foo}", *hash);

    let at_sign = r.at_sign(&reader).unwrap();
    assert_eq!("@foo", *at_sign);
}
