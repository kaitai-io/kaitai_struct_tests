// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::index_to_param_expr::IndexToParamExpr;

#[test]
fn test_index_to_param_expr() {
    let r = IndexToParamExpr::from_file("../../src/index_sizes.bin").expect("file for parsing is not found");

    assert_eq!(r.qty, 3);
    assert_eq!(r.sizes[0], 1);
    assert_eq!(r.sizes[1], 8);
    assert_eq!(r.sizes[2], 4);
    assert_eq!(r.blocks[0].buf, "A");
    assert_eq!(r.blocks[1].buf, "BBBBBBBB");
    assert_eq!(r.blocks[2].buf, "CCCC");
}
