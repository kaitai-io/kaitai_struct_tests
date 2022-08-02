#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::test_parent::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TestParent::default();
    {
        let res = r.read(&reader, None, Some(KStructUnit::parent_stack()));
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    assert_eq!(1, r.root_byte());
    assert_eq!(7, r.child().child_byte());
    assert_eq!(1, r.child().child2().len());
    assert_eq!(vec![2, 64, 64, 4, 55, 19, 0], *r.child().child2()[0].child2_byte());
}
