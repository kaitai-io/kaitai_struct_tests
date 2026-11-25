# expr_int_sign_mismatch_s2_u8.ksy: /instances/value/value:
# 	error: unable to apply binary operator Add to s2 and u8 (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the operands)
#
meta:
  id: expr_int_sign_mismatch_s2_u8
seq:
  - id: s2_val
    type: s2le
  - id: u8_val
    type: u8le
instances:
  value:
    value: s2_val + u8_val
