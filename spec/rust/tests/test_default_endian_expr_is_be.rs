// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai::{BytesReader, KStruct, KStructUnit};
use kaitai_test::default_endian_expr_is_be::*;
use std::fs;

#[test]
fn test_default_endian_expr_is_be() {
    let data = fs::read("../../src/endian_expr.bin").expect("Unable to read data.");
    let reader = BytesReader::new(&data[..]);
    let mut r = DefaultEndianExprIsBe::default();
    r.read(&reader, None, KStructUnit::parent_stack()).expect("Unable to parse data.");
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(0)),identifier(indicator)), List(ArrayBuffer(IntNum(73), IntNum(73))))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(0)),identifier(main)),identifier(some_int)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(0)),identifier(main)),identifier(some_int_be)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(0)),identifier(main)),identifier(some_int_le)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(0)),identifier(main)),identifier(inst_int)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(0)),identifier(main)),identifier(inst_sub)),identifier(foo)), IntNum(66))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(1)),identifier(indicator)), List(ArrayBuffer(IntNum(77), IntNum(77))))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(1)),identifier(main)),identifier(some_int)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(1)),identifier(main)),identifier(some_int_be)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(1)),identifier(main)),identifier(some_int_le)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(1)),identifier(main)),identifier(inst_int)), IntNum(1107296256))
    // assert_eq!(Attribute(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(1)),identifier(main)),identifier(inst_sub)),identifier(foo)), IntNum(1107296256))
    // assert_eq!(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(2)),identifier(indicator)), List(ArrayBuffer(IntNum(88), IntNum(88))))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(2)),identifier(main)),identifier(some_int)), IntNum(1107296256))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(2)),identifier(main)),identifier(some_int_be)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(2)),identifier(main)),identifier(some_int_le)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(2)),identifier(main)),identifier(inst_int)), IntNum(66))
    // assert_eq!(Attribute(Attribute(Attribute(Subscript(Attribute(Name(identifier(q1w2e3)),identifier(docs)),IntNum(2)),identifier(main)),identifier(inst_sub)),identifier(foo)), IntNum(66))
}
