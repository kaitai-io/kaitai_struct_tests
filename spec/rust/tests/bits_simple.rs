// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::bits_simple::BitsSimple;

#[test]
fn test_bits_simple() {
    let r = BitsSimple::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.byte_1, 80);
    assert_eq!(r.byte_2, 65);
    assert_eq!(r.bits_a, false);
    assert_eq!(r.bits_b, 4);
    assert_eq!(r.bits_c, 3);
    assert_eq!(r.large_bits_1, 300);
    assert_eq!(r.spacer, 5);
    assert_eq!(r.large_bits_2, 1329);
    assert_eq!(r.normal_s2, -1);
    assert_eq!(r.byte_8_9_10, 5259587);
    assert_eq!(r.byte_11_to_14, 1261262125);
    assert_eq!(r.byte_15_to_19, 293220057087);
    assert_eq!(r.byte_20_to_27, 18446744073709551615);
    assert_eq!(r.test_if_b1, 123);
}
