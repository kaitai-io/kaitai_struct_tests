// Autogenerated from KST: please remove this line if doing any edits by hand!

use kaitai_struct::KaitaiStruct;
use kaitai_test_suite::expr_calc_array_ops::ExprCalcArrayOps;

#[test]
fn test_expr_calc_array_ops() {
    let r = ExprCalcArrayOps::from_file("../../src/fixed_struct.bin").expect("file for parsing is not found");

    assert_eq!(r.int_array_size, 7);
    assert_eq!(r.int_array_first, 10);
    assert_eq!(r.int_array_mid, 25);
    assert_eq!(r.int_array_last, 1000);
    assert_eq!(r.int_array_min, 10);
    assert_eq!(r.int_array_max, 1000);
    assert_eq!(r.double_array_size, 5);
    assert_eq!(r.double_array_first, 10.0);
    assert_eq!(r.double_array_mid, 25.0);
    assert_eq!(r.double_array_last, 3.14159);
    assert_eq!(r.double_array_min, 3.14159);
    assert_eq!(r.double_array_max, 100.0);
    assert_eq!(r.str_array_size, 4);
    assert_eq!(r.str_array_first, "un");
    assert_eq!(r.str_array_mid, "deux");
    assert_eq!(r.str_array_last, "quatre");
    assert_eq!(r.str_array_min, "deux");
    assert_eq!(r.str_array_max, "un");
}
