# expr_ternary_int_sign_mismatch_lit_lit.ksy: /instances/value/value:
# 	error: unable to combine s4 and u4 in the ternary operator (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the branches)
#
meta:
  id: expr_ternary_int_sign_mismatch_lit_lit
instances:
  value:
    value: 'true ? -1 : 0xffff_ffff'
