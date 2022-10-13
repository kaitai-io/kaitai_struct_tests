use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::opaque_external_type_02_parent::*;

#[test]
fn test_term_strz() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = OpaqueExternalType02Parent::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.parent().child().as_ref().unwrap().s1(), "foo");
    assert_eq!(*r.parent().child().as_ref().unwrap().s2(), "bar");
    assert_eq!(*r.parent().child().as_ref().unwrap().s3().s3(), "|baz@");
}
