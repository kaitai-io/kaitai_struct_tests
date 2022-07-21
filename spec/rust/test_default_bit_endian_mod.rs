// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate kaitai_rust;

use kaitai_struct::KaitaiStruct;
use rust::DefaultBitEndianMod;

#[test]
fn test_default_bit_endian_mod() {
    if let Ok(r) = DefaultBitEndianMod::from_file("src/fixed_struct.bin") {

        assert_eq!(r.main.one, 336);
        assert_eq!(r.main.two, 8608);
        assert_eq!(r.main.nest.two, 11595);
        assert_eq!(r.main.nest_be.two, 12799);
    }
}
