// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::repeat_eos_bit::RepeatEosBit;

#[test]
fn test_repeat_eos_bit() {
    let r = RepeatEosBit::from_file("../../src/enum_0.bin").expect("file for parsing is not found");

    assert_eq!(r.nibbles.len(), 16);
}
