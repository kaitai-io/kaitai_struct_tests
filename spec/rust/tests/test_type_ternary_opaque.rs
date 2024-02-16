use std::fs;

extern crate kaitai;
use self::kaitai::*;
use rust::formats::type_ternary_opaque::*;

#[test]
fn test_type_ternary_opaque() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = TypeTernaryOpaque::read_into(&_io, None, None);
    let r : OptRc<TypeTernaryOpaque>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    let dif = r.dif().unwrap();
    assert_eq!(*dif.s1(), "foo");
    assert_eq!(*dif.s2(), "bar");
    assert_eq!(*dif.s3(), "|baz@");
}
