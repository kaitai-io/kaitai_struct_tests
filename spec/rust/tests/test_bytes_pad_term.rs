// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::bytes_pad_term::BytesPadTerm;
use std::fs;

#[test]
fn test_bytes_pad_term() {
    let data = fs::read("../../src/str_pad_term.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = BytesPadTerm::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.str_pad, vec!([0x73, 0x74, 0x72, 0x31]));
    // assert_eq!(r.str_term, vec!([0x73, 0x74, 0x72, 0x32, 0x66, 0x6f, 0x6f]));
    // assert_eq!(r.str_term_and_pad, vec!([0x73, 0x74, 0x72, 0x2b, 0x2b, 0x2b, 0x33, 0x62, 0x61, 0x72, 0x2b, 0x2b, 0x2b]));
    // assert_eq!(r.str_term_include, vec!([0x73, 0x74, 0x72, 0x34, 0x62, 0x61, 0x7a, 0x40]));
}
