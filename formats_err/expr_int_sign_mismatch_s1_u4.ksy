# expr_int_sign_mismatch_s1_u4.ksy: /instances/value/value:
# 	error: unable to apply binary operator Sub to s1 and u4 (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the operands)
#
meta:
  id: expr_int_sign_mismatch_s1_u4
seq:
  - id: s1_val
    type: s1
  - id: u4_val
    type: u4le
instances:
  value:
    value: s1_val - u4_val
