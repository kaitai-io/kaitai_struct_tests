# expr_ternary_int_sign_mismatch_u8_s1.ksy: /instances/value/value:
# 	error: unable to combine u8 and s1 in the ternary operator (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the branches)
#
meta:
  id: expr_ternary_int_sign_mismatch_u8_s1
seq:
  - id: u8_val
    type: u8le
  - id: s1_val
    type: s1
instances:
  value:
    value: 'true ? u8_val : s1_val'
