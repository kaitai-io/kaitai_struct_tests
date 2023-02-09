use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::opaque_external_type_02_parent::*;

#[test]
fn test_term_strz() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = OpaqueExternalType02Parent::read_into(&_io, None, None);
    let r : OptRc<OpaqueExternalType02Parent>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.parent().child().s1(), "foo");
    assert_eq!(*r.parent().child().s2(), "bar");
    assert_eq!(*r.parent().child().s3().s3(), "|baz@");
}
