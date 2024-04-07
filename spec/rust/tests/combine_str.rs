// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::combine_str::CombineStr;

#[test]
fn test_combine_str() {
    let r = CombineStr::from_file("../../src/term_strz.bin").expect("file for parsing is not found");

    assert_eq!(r.str_term, "foo");
    assert_eq!(r.str_limit, "bar|");
    assert_eq!(r.str_eos, "baz@");
    assert_eq!(r.str_calc, "bar");
    assert_eq!(r.str_calc_bytes, "baz");
    assert_eq!(r.term_or_limit, "foo");
    assert_eq!(r.term_or_eos, "baz@");
    assert_eq!(r.term_or_calc, "foo");
    assert_eq!(r.term_or_calc_bytes, "baz");
    assert_eq!(r.limit_or_eos, "bar|");
    assert_eq!(r.limit_or_calc, "bar");
    assert_eq!(r.limit_or_calc_bytes, "bar|");
    assert_eq!(r.eos_or_calc, "bar");
    assert_eq!(r.eos_or_calc_bytes, "baz@");
    assert_eq!(r.calc_or_calc_bytes, "baz");
}
