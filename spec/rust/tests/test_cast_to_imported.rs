use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::cast_to_imported::*;

#[test]
fn test_cast_to_imported() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = CastToImported::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*(*(*r.one()).as_ref().unwrap()).one(), 80);
    assert_eq!(*(*(*r.one_casted(&reader, Some(&r)).unwrap()).as_ref().unwrap()).one(), 80);
}
