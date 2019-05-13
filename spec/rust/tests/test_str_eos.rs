// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::str_eos::StrEos;
use std::fs;

#[test]
fn test_str_eos() {
    let data = fs::read("../../src/term_strz.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = StrEos::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.str, "foo|bar|baz@");
}
