use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::expr_int_div::*;

#[test]
fn test_expr_int_div() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res = ExprIntDiv::read_into(&_io, None, None);
    let r : Rc<ExprIntDiv>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.int_u(), 1262698832);
    assert_eq!(*r.int_s(), -52947);
    assert_eq!(*r.div_pos_const().unwrap(), 756);
    assert_eq!(*r.div_neg_const().unwrap(), -756);
    assert_eq!(*r.div_pos_seq().unwrap(), 97130679);
    assert_eq!(*r.div_neg_seq().unwrap(), -4072);
}
