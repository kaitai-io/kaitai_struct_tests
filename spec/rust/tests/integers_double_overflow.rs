// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::integers_double_overflow::IntegersDoubleOverflow;

#[test]
fn test_integers_double_overflow() {
    let r = IntegersDoubleOverflow::from_file("../../src/integers_double_overflow.bin").expect("file for parsing is not found");

    assert_eq!(r.signed_safe_min_be, -9007199254740991);
    assert_eq!(r.signed_safe_min_le, -9007199254740991);
    assert_eq!(r.signed_safe_max_be, 9007199254740991);
    assert_eq!(r.signed_safe_max_le, 9007199254740991);
    assert_eq!(r.signed_unsafe_neg_be.to_string(), "-9007199254740993");
    assert_eq!(r.signed_unsafe_neg_le.to_string(), "-9007199254740993");
    assert_eq!(r.signed_unsafe_pos_be.to_string(), "9007199254740993");
    assert_eq!(r.signed_unsafe_pos_be.to_string(), "9007199254740993");
}
