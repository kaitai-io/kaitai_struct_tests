// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::expr_io_eof::ExprIoEof;

#[test]
fn test_expr_io_eof() {
    let r = ExprIoEof::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.substream1.one, 1262698832);
    assertNull(r.substream1.two);
    assert_eq!(r.substream2.one, 4294914349);
    assert_eq!(r.substream2.two, 1262698832);
}
