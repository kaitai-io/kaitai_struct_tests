// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::expr_2::Expr2;
use std::fs;

#[test]
fn test_expr_2() {
    let data = fs::read("../../src/str_encodings.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = Expr2::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.str1.len_orig, 10);
    // assert_eq!(r.str1.len_mod, 7);
    // assert_eq!(r.str1.str, "Some AS");
    // assert_eq!(r.str1_len, 7);
    // assert_eq!(r.str1_len_mod, 7);
    // assert_eq!(r.str1_byte1, 73);
    // assert_eq!(r.str1_avg, 73);
    // assert_eq!(r.str1_char5, "e");
    // assert_eq!(r.str1_tuple5.byte0, 101);
    // assert_eq!(r.str1_tuple5.byte0, 101);
    // assert_eq!(r.str1_tuple5.byte1, 32);
    // assert_eq!(r.str1_tuple5.byte2, 65);
    // assert_eq!(r.str1_tuple5.avg, 48);
    // assert_eq!(r.str2_tuple5.byte0, 101);
    // assert_eq!(r.str2_tuple5.byte1, 32);
    // assert_eq!(r.str2_tuple5.byte2, 65);
    // assert_eq!(r.str2_tuple5.avg, 48);
}
