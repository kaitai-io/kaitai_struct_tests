use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::type_ternary_2nd_falsy::*;


#[test]
fn test_type_ternary_2nd_falsy() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let _io = BytesReader::new(&bytes);
    let res = TypeTernary2ndFalsy::read_into(&_io, None, None);
    let r : Rc<TypeTernary2ndFalsy>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.v_false(&_io).unwrap(), false);
    assert_eq!(*r.v_int_zero(&_io).unwrap(), 0);
    assert_eq!(*r.v_int_neg_zero(&_io).unwrap(), 0);
    assert_eq!(*r.v_float_zero(&_io).unwrap(), 0.0);
    assert_eq!(*r.v_float_neg_zero(&_io).unwrap(), -0.0);
    assert_eq!(*r.v_str_w_zero(&_io).unwrap(), "0");
    assert_eq!(r.v_str_w_zero(&_io).unwrap().len(), 1);
    assert_eq!(*r.ut().m(), 7);
    assert_eq!(*r.v_null_ut(&_io).unwrap().m(), 0);
    assert_eq!(*r.v_str_empty(&_io).unwrap(), "");
    assert_eq!(r.v_str_empty(&_io).unwrap().len(), 0);
    assert_eq!(r.int_array().len(), 2);
    assert_eq!(r.v_int_array_empty(&_io).unwrap().len(), 0);
}
