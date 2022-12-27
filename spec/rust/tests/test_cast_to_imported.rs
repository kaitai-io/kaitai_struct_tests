use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::cast_to_imported::*;

#[test]
fn test_cast_to_imported() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = CastToImported::read_into(&reader, None, None);
    let r : Rc<CastToImported>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*(*(*r.one()).as_ref().unwrap()).one(), 80);
    assert_eq!(*(*(*r.one_casted(&reader).unwrap()).as_ref().unwrap()).one(), 80);
}
