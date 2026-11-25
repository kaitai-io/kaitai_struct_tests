# expr_int_sign_mismatch_lit_lit.ksy: /instances/value/value:
# 	error: unable to apply binary operator Add to s4 and u4 (signed integers can't be mixed with unsigned integers of the same or greater width, use a type cast `.as<>` on one of the operands)
#
meta:
  id: expr_int_sign_mismatch_lit_lit
instances:
  value:
    value: -1 + 0xffff_ffff
