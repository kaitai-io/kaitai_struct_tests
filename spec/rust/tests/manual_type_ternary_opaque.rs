use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::type_ternary_opaque::*;

#[test]
fn test_term_strz() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = TypeTernaryOpaque::read_into(&_io, None, None);
    let r : Rc<TypeTernaryOpaque>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    let dif = r.dif().unwrap();
    assert_eq!(*dif.as_ref().unwrap().s1(), "foo");
    assert_eq!(*dif.as_ref().unwrap().s2(), "bar");
    assert_eq!(*dif.as_ref().unwrap().s3(), "|baz@");
}
