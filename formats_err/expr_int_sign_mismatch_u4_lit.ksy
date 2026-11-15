# expr_int_sign_mismatch_u4_lit.ksy: /instances/value/value:
# 	error: unable to apply binary operator Add to u4 and s4 (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the operands)
#
meta:
  id: expr_int_sign_mismatch_u4_lit
seq:
  - id: u4_val
    type: u4le
instances:
  value:
    value: u4_val + -1
