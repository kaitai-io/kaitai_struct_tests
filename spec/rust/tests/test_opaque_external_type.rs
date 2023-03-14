use std::fs;

extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::opaque_external_type::*;

#[test]
fn test_opaque_external_type() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = OpaqueExternalType::read_into(&_io, None, None);
    let r : OptRc<OpaqueExternalType>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one().s1(), "foo");
    assert_eq!(*r.one().s2(), "bar");
    assert_eq!(*r.one().s3(), "|baz@");
}
