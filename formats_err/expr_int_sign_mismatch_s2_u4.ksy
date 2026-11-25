# expr_int_sign_mismatch_s2_u4.ksy: /instances/value/value:
# 	error: unable to apply binary operator BitOr to s2 and u4 (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the operands)
#
meta:
  id: expr_int_sign_mismatch_s2_u4
seq:
  - id: s2_val
    type: s2le
  - id: u4_val
    type: u4le
instances:
  value:
    value: s2_val | u4_val
