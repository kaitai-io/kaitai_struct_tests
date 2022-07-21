// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate kaitai_rust;

use kaitai_struct::KaitaiStruct;
use rust::BitsSeqEndianCombo;

#[test]
fn test_bits_seq_endian_combo() {
    if let Ok(r) = BitsSeqEndianCombo::from_file("src/process_xor_4.bin") {

        assert_eq!(r.be1, 59);
        assert_eq!(r.be2, 187);
        assert_eq!(r.le3, 163);
        assert_eq!(r.be4, 20);
        assert_eq!(r.le5, 10);
        assert_eq!(r.le6, 36);
        assert_eq!(r.le7, 26);
        assert_eq!(r.be8, true);
    }
}
