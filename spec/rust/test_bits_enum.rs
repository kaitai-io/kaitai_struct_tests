// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate kaitai_rust;

use kaitai_struct::KaitaiStruct;
use rust::BitsEnum;

#[test]
fn test_bits_enum() {
    if let Ok(r) = BitsEnum::from_file("src/fixed_struct.bin") {

        assert_eq!(r.one, BitsEnum_Animal::Platypus);
        assert_eq!(r.two, BitsEnum_Animal::Horse);
        assert_eq!(r.three, BitsEnum_Animal::Cat);
    }
}
