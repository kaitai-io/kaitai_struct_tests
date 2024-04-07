// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::expr_3::Expr3;

#[test]
fn test_expr_3() {
    let r = Expr3::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.one, 80);
    assert_eq!(r.two, "ACK");
    assert_eq!(r.three, "@ACK");
    assert_eq!(r.four, "_ACK_");
    assert_eq!(r.is_str_eq, true);
    assert_eq!(r.is_str_ne, false);
    assert_eq!(r.is_str_lt, true);
    assert_eq!(r.is_str_gt, false);
    assert_eq!(r.is_str_le, true);
    assert_eq!(r.is_str_ge, false);
    assert_eq!(r.is_str_lt2, true);
    assert_eq!(r.test_not, true);
}
