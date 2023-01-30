#![allow(dead_code)]
use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::test_parent::*;

#[test]
fn basic_parse() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = TestParent::read_into(&_io, None, None);
    let r : Rc<TestParent>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(1, *r.root_byte());
    assert_eq!(7, *r.child().child_byte());
    assert_eq!(1, r.child().child2().len());
    assert_eq!(vec![2, 64, 64, 4, 55, 19, 0], *r.child().child2()[0].child2_byte());
    assert_eq!(1, r.child().child2()[0].child3_struct().len());
    assert_eq!(vec![0], *r.child().child2()[0].child3_struct()[0].child3_byte());
}
