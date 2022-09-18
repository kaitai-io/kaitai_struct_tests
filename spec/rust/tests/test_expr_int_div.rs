use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::expr_int_div::*;

#[test]
fn test_expr_int_div() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = ExprIntDiv::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(*r.int_u(), 1262698832);
    assert_eq!(*r.int_s(), -52947);
    assert_eq!(*r.div_pos_const(&reader, Some(&r)).unwrap(), 756);
    assert_eq!(*r.div_neg_const(&reader, Some(&r)).unwrap(), -756);
    assert_eq!(*r.div_pos_seq(&reader, Some(&r)).unwrap(), 97130679);
    assert_eq!(*r.div_neg_seq(&reader, Some(&r)).unwrap(), -4072);
}
