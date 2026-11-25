# expr_ternary_int_sign_mismatch_u4_lit.ksy: /instances/value/value:
# 	error: unable to combine u4 and s4 in the ternary operator (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the branches)
#
meta:
  id: expr_ternary_int_sign_mismatch_u4_lit
seq:
  - id: u4_val
    type: u4le
instances:
  value:
    value: 'true ? u4_val : -1'
