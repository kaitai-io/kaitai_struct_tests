#![allow(dead_code)]
use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::str_encodings_utf16::*;

#[test]
fn test_str_encodings_utf16() {
    let bytes = fs::read("../../src/str_encodings_utf16.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = StrEncodingsUtf16::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {

        panic!("{:?}", err);
    }
    assert_eq!(r.len_be(), 12);
    assert_eq!(r.be_bom_removed().bom(), 65279);
    assert_eq!(r.be_bom_removed().str(), "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}");
    assert_eq!(r.len_le(), 12);
    assert_eq!(r.le_bom_removed().bom(), 65279);
    assert_eq!(r.le_bom_removed().str(), "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}");
}
