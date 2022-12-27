use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::expr_int_div::*;

#[test]
fn test_expr_int_div() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let res = ExprIntDiv::read_into(&reader, None, None);
    let r : Rc<ExprIntDiv>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.int_u(), 1262698832);
    assert_eq!(*r.int_s(), -52947);
    assert_eq!(*r.div_pos_const(&reader).unwrap(), 756);
    assert_eq!(*r.div_neg_const(&reader).unwrap(), -756);
    assert_eq!(*r.div_pos_seq(&reader).unwrap(), 97130679);
    assert_eq!(*r.div_neg_seq(&reader).unwrap(), -4072);
}
