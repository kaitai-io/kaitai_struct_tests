use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::imports_abs_rel::*;


#[test]
fn test_params_def() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ImportsAbsRel::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.one(), 80);
    assert_eq!(*r.two().as_ref().unwrap().one(), 65);
    assert_eq!(*r.two().as_ref().unwrap().two().as_ref().unwrap().one(), 67);
}
