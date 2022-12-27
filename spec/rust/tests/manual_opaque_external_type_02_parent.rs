use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::opaque_external_type_02_parent::*;

#[test]
fn test_term_strz() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = OpaqueExternalType02Parent::read_into(&reader, None, None);
    let r : Rc<OpaqueExternalType02Parent>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.parent().child().as_ref().unwrap().s1(), "foo");
    assert_eq!(*r.parent().child().as_ref().unwrap().s2(), "bar");
    assert_eq!(*r.parent().child().as_ref().unwrap().s3().s3(), "|baz@");
}
