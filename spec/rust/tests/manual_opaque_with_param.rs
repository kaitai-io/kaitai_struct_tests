use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::opaque_with_param::*;


#[test]
fn test_params_def() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = OpaqueWithParam::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    let one = r.one();
    let p = one.as_ref().unwrap();
    assert_eq!(*p.buf(), "foo|b");
    assert_eq!(*p.trailer(), 0x61);
}
