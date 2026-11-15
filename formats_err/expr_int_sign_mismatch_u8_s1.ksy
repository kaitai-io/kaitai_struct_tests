# expr_int_sign_mismatch_u8_s1.ksy: /instances/value/value:
# 	error: unable to apply binary operator Mult to u8 and s1 (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the operands)
#
meta:
  id: expr_int_sign_mismatch_u8_s1
seq:
  - id: u8_val
    type: u8le
  - id: s1_val
    type: s1
instances:
  value:
    value: u8_val * s1_val
