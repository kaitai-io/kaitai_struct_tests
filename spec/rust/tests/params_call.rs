// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::params_call::ParamsCall;

#[test]
fn test_params_call() {
    let r = ParamsCall::from_file("../../src/term_strz.bin").expect("file for parsing is not found");

    assert_eq!(r.buf1.body, "foo|b");
    assert_eq!(r.buf2.body, "ar|ba");
    assert_eq!(r.buf2.trailer, 122);
}
