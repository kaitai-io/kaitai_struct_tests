use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::imports_abs_rel::*;


#[test]
fn test_params_def() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = ImportsAbsRel::read_into(&_io, None, None);
    let r : Rc<ImportsAbsRel>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one(), 80);
    assert_eq!(*r.two().as_ref().unwrap().one(), 65);
    assert_eq!(*r.two().as_ref().unwrap().two().as_ref().unwrap().one(), 67);
}
