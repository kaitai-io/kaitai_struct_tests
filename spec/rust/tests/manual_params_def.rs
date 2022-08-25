use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::params_def::*;


#[test]
fn test_params_def() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ParamsDef::default();

    r.set_params(5, true);

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.buf(), "foo|b");
    assert_eq!(*r.trailer(), 0x61);
}
