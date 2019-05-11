// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::expr_0::Expr0;
use std::fs;

#[test]
fn test_expr_0() {
    let data = fs::read("src/str_encodings.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = Expr0::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.must_be_f7, 247);
    // assert_eq!(r.must_be_abc123, "abc123");
}
