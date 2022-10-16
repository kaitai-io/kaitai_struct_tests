use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::type_ternary_opaque::*;

#[test]
fn test_term_strz() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TypeTernaryOpaque::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    let dif = r.dif(&reader, Some(&r)).unwrap();
    assert_eq!(*dif.as_ref().unwrap().s1(), "foo");
    assert_eq!(*dif.as_ref().unwrap().s2(), "bar");
    assert_eq!(*dif.as_ref().unwrap().s3(), "|baz@");
}
