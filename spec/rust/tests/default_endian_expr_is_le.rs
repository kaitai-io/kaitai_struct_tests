// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::default_endian_expr_is_le::DefaultEndianExprIsLe;

#[test]
fn test_default_endian_expr_is_le() {
    let r = DefaultEndianExprIsLe::from_file("../../src/endian_expr.bin").expect("file for parsing is not found");

    assert_eq!(r.docs[0].indicator, vec!([0x49, 0x49]));
    assert_eq!(r.docs[0].main.some_int, 66);
    assert_eq!(r.docs[0].main.some_int_be, 66);
    assert_eq!(r.docs[0].main.some_int_le, 66);
    assert_eq!(r.docs[1].indicator, vec!([0x4d, 0x4d]));
    assert_eq!(r.docs[1].main.some_int, 66);
    assert_eq!(r.docs[1].main.some_int_be, 66);
    assert_eq!(r.docs[1].main.some_int_le, 66);
    assert_eq!(r.docs[2].indicator, vec!([0x58, 0x58]));
    assert_eq!(r.docs[2].main.some_int, 66);
    assert_eq!(r.docs[2].main.some_int_be, 66);
    assert_eq!(r.docs[2].main.some_int_le, 66);
}
