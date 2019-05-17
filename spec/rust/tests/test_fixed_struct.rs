// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::fixed_struct::*;
use std::fs;

#[test]
fn test_fixed_struct() {
    let data = fs::read("../../src/fixed_struct.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = FixedStruct::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint8)), IntNum(255))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint16)), IntNum(65535))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint32)), IntNum(4294967295))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint64)), IntNum(18446744073709551615))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint8)), UnaryOp(Minus,IntNum(1)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint16)), UnaryOp(Minus,IntNum(1)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint32)), UnaryOp(Minus,IntNum(1)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint64)), UnaryOp(Minus,IntNum(1)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint16le)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint32le)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint64le)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint16le)), UnaryOp(Minus,IntNum(66)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint32le)), UnaryOp(Minus,IntNum(66)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint64le)), UnaryOp(Minus,IntNum(66)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint16be)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint32be)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(uint64be)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint16be)), UnaryOp(Minus,IntNum(66)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint32be)), UnaryOp(Minus,IntNum(66)))
    // assert_eq!(Attribute(Attribute(Name(identifier(q1w2e3)),identifier(hdr)),identifier(sint64be)), UnaryOp(Minus,IntNum(66)))
}
