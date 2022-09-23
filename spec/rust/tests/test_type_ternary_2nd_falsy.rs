use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::type_ternary_2nd_falsy::*;


#[test]
fn test_type_ternary_2nd_falsy() {
    let bytes = fs::read("../../src/switch_integers.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = TypeTernary2ndFalsy::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.v_false(&reader, Some(&r)).unwrap(), false);
    assert_eq!(*r.v_int_zero(&reader, Some(&r)).unwrap(), 0);
    assert_eq!(*r.v_int_neg_zero(&reader, Some(&r)).unwrap(), 0);
    assert_eq!(*r.v_float_zero(&reader, Some(&r)).unwrap(), 0.0);
    assert_eq!(*r.v_float_neg_zero(&reader, Some(&r)).unwrap(), -0.0);
    assert_eq!(*r.v_str_w_zero(&reader, Some(&r)).unwrap(), "0");
    assert_eq!(r.v_str_w_zero(&reader, Some(&r)).unwrap().len(), 1);
    assert_eq!(*r.ut().m(), 7);
    assert_eq!(*r.v_null_ut(&reader, Some(&r)).unwrap().m(), 0);
    assert_eq!(*r.v_str_empty(&reader, Some(&r)).unwrap(), "");
    assert_eq!(r.v_str_empty(&reader, Some(&r)).unwrap().len(), 0);
    assert_eq!(r.int_array().len(), 2);
    assert_eq!(r.v_int_array_empty(&reader, Some(&r)).unwrap().len(), 0);
}
