// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::bcd_user_type_le::BcdUserTypeLe;

#[test]
fn test_bcd_user_type_le() {
    let r = BcdUserTypeLe::from_file("../../src/bcd_user_type_le.bin").expect("file for parsing is not found");

    assert_eq!(r.ltr.as_int, 12345678);
    assert_eq!(r.ltr.as_str, "12345678");
    assert_eq!(r.rtl.as_int, 87654321);
    assert_eq!(r.rtl.as_str, "87654321");
    assert_eq!(r.leading_zero_ltr.as_int, 123456);
    assert_eq!(r.leading_zero_ltr.as_str, "00123456");
}
