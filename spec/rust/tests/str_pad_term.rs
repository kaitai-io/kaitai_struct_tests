// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::str_pad_term::StrPadTerm;

#[test]
fn test_str_pad_term() {
    let r = StrPadTerm::from_file("../../src/str_pad_term.bin").expect("file for parsing is not found");

    assert_eq!(r.str_pad, "str1");
    assert_eq!(r.str_term, "str2foo");
    assert_eq!(r.str_term_and_pad, "str+++3bar+++");
    assert_eq!(r.str_term_include, "str4baz@");
}
