// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::enum_int_range_s::EnumIntRangeS;

#[test]
fn test_enum_int_range_s() {
    let r = EnumIntRangeS::from_file("../../src/enum_int_range_s.bin").expect("file for parsing is not found");

    assert_eq!(r.f1, EnumIntRangeS__Constants::INT_MIN);
    assert_eq!(r.f2, EnumIntRangeS__Constants::ZERO);
    assert_eq!(r.f3, EnumIntRangeS__Constants::INT_MAX);
}
