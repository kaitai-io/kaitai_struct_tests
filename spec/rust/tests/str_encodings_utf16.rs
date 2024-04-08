// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::str_encodings_utf16::StrEncodingsUtf16;

#[test]
fn test_str_encodings_utf16() {
    let r = StrEncodingsUtf16::from_file("../../src/str_encodings_utf16.bin").expect("file for parsing is not found");

    assert_eq!(r.len_be, 12);
    assert_eq!(r.be_bom_removed.bom, 65279);
    assert_eq!(r.be_bom_removed.str, "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}");
    assert_eq!(r.len_le, 12);
    assert_eq!(r.le_bom_removed.bom, 65279);
    assert_eq!(r.le_bom_removed.str, "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}");
}
