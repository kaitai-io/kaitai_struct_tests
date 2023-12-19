use std::fs;

extern crate kaitai;
use self::kaitai::*;
use rust::formats::imports_abs_rel::*;

#[test]
fn test_imports_abs_rel() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = ImportsAbsRel::read_into(&_io, None, None);
    let r : OptRc<ImportsAbsRel>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one(), 80);
    assert_eq!(*r.two().one(), 65);
    assert_eq!(*r.two().two().one(), 67);
}
