// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::ExprMod;

#[test]
fn test_expr_mod() {
    if let Ok(r) = ExprMod::from_file("src/fixed_struct.bin") {
        assert_eq!(r.int_u, 1262698832);
        assert_eq!(r.int_s, -52947);
        assert_eq!(r.mod_pos_const, 9);
        assert_eq!(r.mod_neg_const, 4);
        assert_eq!(r.mod_pos_seq, 5);
        assert_eq!(r.mod_neg_seq, 2);
    }
}
