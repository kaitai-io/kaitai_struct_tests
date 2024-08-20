// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::expr_sizeof_value_0::*;

#[test]
fn test_expr_sizeof_value_0() -> KResult<()> {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<ExprSizeofValue0> = ExprSizeofValue0::read_into(&_io, None, None)?;

    assert_eq!(*r.self_sizeof()?, (((((1 + 4) as i32) + (2 as i32)) as i32) + (2 as i32)));
    assert_eq!(*r.sizeof_block()?, (((1 + 4) as i32) + (2 as i32)));
    assert_eq!(*r.sizeof_block_a()?, 1);
    assert_eq!(*r.sizeof_block_b()?, 4);
    assert_eq!(*r.sizeof_block_c()?, 2);
    Ok(())
}