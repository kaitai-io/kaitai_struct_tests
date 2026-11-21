# expr_ternary_int_sign_mismatch_s4_u8.ksy: /instances/value/value:
# 	error: unable to combine s4 and u8 in the ternary operator (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the branches)
#
meta:
  id: expr_ternary_int_sign_mismatch_s4_u8
seq:
  - id: s4_val
    type: s4le
  - id: u8_val
    type: u8le
instances:
  value:
    value: 'true ? s4_val : u8_val'
