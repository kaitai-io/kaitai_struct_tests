// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::str_eos::StrEos;

#[test]
fn test_str_eos() {
    let r = StrEos::from_file("../../src/term_strz.bin").expect("file for parsing is not found");

    assert_eq!(r.str, "foo|bar|baz@");
}
