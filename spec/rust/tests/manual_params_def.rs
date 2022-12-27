use std::{fs, rc::Rc};

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::params_def::*;


#[test]
fn test_params_def() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let f = |t: &mut ParamsDef| t.set_params(5, true);
    let res = ParamsDef::read_into_with_init(&reader, None, None, &f);
    let r : Rc<ParamsDef>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.buf(), "foo|b");
    assert_eq!(*r.trailer(), 0x61);
}
