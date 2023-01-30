use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::opaque_external_type::*;

#[test]
fn test_term_strz() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = OpaqueExternalType::read_into(&_io, None, None);
    let r : Rc<OpaqueExternalType>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one().as_ref().unwrap().s1(), "foo");
    assert_eq!(*r.one().as_ref().unwrap().s2(), "bar");
    assert_eq!(*r.one().as_ref().unwrap().s3(), "|baz@");
}
