use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::imports_circular_a::*;

#[test]
fn test_imports_circular_a() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = ImportsCircularA::read_into(&reader, None, None);
    let r : Rc<ImportsCircularA>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.code(), 80);
    assert_eq!(*r.two().as_ref().unwrap().initial(), 65);
    assert_eq!(*r.two().as_ref().unwrap().back_ref().as_ref().unwrap().code(), 67);
    assert_eq!(*r.two().as_ref().unwrap().back_ref().as_ref().unwrap().two().as_ref().unwrap().initial(), 75);
    assert_eq!(r.two().as_ref().unwrap().back_ref().as_ref().unwrap().two().as_ref().unwrap().back_ref().is_none(), true);
}
