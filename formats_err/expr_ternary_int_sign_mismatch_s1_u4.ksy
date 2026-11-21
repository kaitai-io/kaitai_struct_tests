# expr_ternary_int_sign_mismatch_s1_u4.ksy: /instances/value/value:
# 	error: unable to combine s1 and u4 in the ternary operator (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the branches)
#
meta:
  id: expr_ternary_int_sign_mismatch_s1_u4
seq:
  - id: s1_val
    type: s1
  - id: u4_val
    type: u4le
instances:
  value:
    value: 'true ? s1_val : u4_val'
