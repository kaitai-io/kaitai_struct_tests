// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::DefaultEndianExprIsBe;

#[test]
fn test_default_endian_expr_is_be() {
    if let Ok(r) = DefaultEndianExprIsBe::from_file("src/endian_expr.bin") {
        assert_eq!(r.docs[0].indicator, vec!([0x49, 0x49]));
        assert_eq!(r.docs[0].main.some_int, 66);
        assert_eq!(r.docs[0].main.some_int_be, 66);
        assert_eq!(r.docs[0].main.some_int_le, 66);
        assert_eq!(r.docs[0].main.inst_int, 66);
        assert_eq!(r.docs[0].main.inst_sub.foo, 66);
        assert_eq!(r.docs[1].indicator, vec!([0x4d, 0x4d]));
        assert_eq!(r.docs[1].main.some_int, 66);
        assert_eq!(r.docs[1].main.some_int_be, 66);
        assert_eq!(r.docs[1].main.some_int_le, 66);
        assert_eq!(r.docs[1].main.inst_int, 1107296256);
        assert_eq!(r.docs[1].main.inst_sub.foo, 1107296256);
        assert_eq!(r.docs[2].indicator, vec!([0x58, 0x58]));
        assert_eq!(r.docs[2].main.some_int, 1107296256);
        assert_eq!(r.docs[2].main.some_int_be, 66);
        assert_eq!(r.docs[2].main.some_int_le, 66);
        assert_eq!(r.docs[2].main.inst_int, 66);
        assert_eq!(r.docs[2].main.inst_sub.foo, 66);
    }
}
