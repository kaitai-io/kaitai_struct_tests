// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::switch_integers2::*;
use std::fs;

#[test]
fn test_switch_integers2() {
    let data = fs::read("../../src/switch_integers.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = SwitchIntegers2::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.code, 1);
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(len)), IntNum(7))
    assert_eq!(r.ham, &[0x2, 0x40, 0x40, 0x4, 0x37, 0x13, 0x0]);
    assert_eq!(r.padding, 0);
    // assert_eq!(Attribute(Name(identifier(q1w2e3)),identifier(len_mod_str)), Str(13))
}
