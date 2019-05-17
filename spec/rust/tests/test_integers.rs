// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::integers::*;
use std::fs;

#[test]
fn test_integers() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = Integers::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    assert_eq!(r.uint8, 255);
    assert_eq!(r.uint16, 65535);
    assert_eq!(r.uint32, 4294967295);
    assert_eq!(r.uint64, 18446744073709551615);
    assert_eq!(r.sint8, -1);
    assert_eq!(r.sint16, -1);
    assert_eq!(r.sint32, -1);
    assert_eq!(r.sint64, -1);
    assert_eq!(r.uint16le, 66);
    assert_eq!(r.uint32le, 66);
    assert_eq!(r.uint64le, 66);
    assert_eq!(r.sint16le, -66);
    assert_eq!(r.sint32le, -66);
    assert_eq!(r.sint64le, -66);
    assert_eq!(r.uint16be, 66);
    assert_eq!(r.uint32be, 66);
    assert_eq!(r.uint64be, 66);
    assert_eq!(r.sint16be, -66);
    assert_eq!(r.sint32be, -66);
    assert_eq!(r.sint64be, -66);
}
