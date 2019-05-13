// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::fixed_struct::FixedStruct;
use std::fs;

#[test]
fn test_fixed_struct() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let r = FixedStruct::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();
    // assert_eq!(r.hdr.uint8, 255);
    // assert_eq!(r.hdr.uint16, 65535);
    // assert_eq!(r.hdr.uint32, 4294967295);
    // assert_eq!(r.hdr.uint64, 18446744073709551615);
    // assert_eq!(r.hdr.sint8, -1);
    // assert_eq!(r.hdr.sint16, -1);
    // assert_eq!(r.hdr.sint32, -1);
    // assert_eq!(r.hdr.sint64, -1);
    // assert_eq!(r.hdr.uint16le, 66);
    // assert_eq!(r.hdr.uint32le, 66);
    // assert_eq!(r.hdr.uint64le, 66);
    // assert_eq!(r.hdr.sint16le, -66);
    // assert_eq!(r.hdr.sint32le, -66);
    // assert_eq!(r.hdr.sint64le, -66);
    // assert_eq!(r.hdr.uint16be, 66);
    // assert_eq!(r.hdr.uint32be, 66);
    // assert_eq!(r.hdr.uint64be, 66);
    // assert_eq!(r.hdr.sint16be, -66);
    // assert_eq!(r.hdr.sint32be, -66);
    // assert_eq!(r.hdr.sint64be, -66);
}
