// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::enum_invalid::*;


#[test]
fn test_enum_invalid() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = EnumInvalid::read_into(&reader, None, None);
    let r : Rc<EnumInvalid>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.pet_1(), EnumInvalid_Animal::Dog);
    let i : i64 = (&*r.pet_2()).into();
    assert_eq!(i, 111);
}