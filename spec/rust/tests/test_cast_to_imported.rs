use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::cast_to_imported::*;

#[test]
fn test_cast_to_imported() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = CastToImported::read_into(&_io, None, None);
    let r : OptRc<CastToImported>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*(*(*r.one())).one(), 80);
    assert_eq!(*(*(*r.one_casted().unwrap())).one(), 80);
}
