extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::OptionalId;

#[test]
fn test_optional_id() {
    if let Ok(r) = OptionalId::from_file("src/fixed_struct.bin") {

        assert_eq!(r._unnamed0, 80);
        assert_eq!(r._unnamed1, 65);
        assert_eq!(r._unnamed2, vec!([0x43, 0x4b, 0x2d, 0x31, 0xff]));
    }
}
