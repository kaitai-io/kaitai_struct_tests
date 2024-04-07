// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::params_pass_struct::ParamsPassStruct;

#[test]
fn test_params_pass_struct() {
    let r = ParamsPassStruct::from_file("../../src/enum_negative.bin").expect("file for parsing is not found");

    assert_eq!(r.first.foo, 255);
    assert_eq!(r.one.bar.qux, 1);
    assert_eq!(r.one.foo.foo, 255);
    assert_eq!(r.one.bar.foo.foo, 255);
}
