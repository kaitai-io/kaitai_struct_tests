// Autogenerated from KST: please remove this line if doing any edits by hand!

extern crate kaitai_struct;
extern crate rust;

use kaitai_struct::KaitaiStruct;
use rust::ParamsPassArrayInt;

#[test]
fn test_params_pass_array_int() {
    if let Ok(r) = ParamsPassArrayInt::from_file("src/position_to_end.bin") {

        assert_eq!(r.pass_ints.nums.len(), 3);
        assert_eq!(r.pass_ints.nums[0], 513);
        assert_eq!(r.pass_ints.nums[1], 1027);
        assert_eq!(r.pass_ints.nums[2], 1541);
        assert_eq!(r.pass_ints_calc.nums.len(), 2);
        assert_eq!(r.pass_ints_calc.nums[0], 27643);
        assert_eq!(r.pass_ints_calc.nums[1], 7);
    }
}