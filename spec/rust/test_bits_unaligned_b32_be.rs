// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::BitsUnalignedB32Be;

#[test]
fn test_bits_unaligned_b32_be() {
    if let Ok(r) = BitsUnalignedB32Be::from_file("src/process_xor_4.bin") {

        assert_eq!(r.a, true);
        assert_eq!(r.b, 3648472617);
        assert_eq!(r.c, 10);
    }
}
