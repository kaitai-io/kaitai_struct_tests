# expr_shift_distance_eq_width_u4.ksy: /instances/value/value:
# 	error: shift distance 32 is out of range for `<<` on u4: the result type is u4, so the distance must be in 0..31 (to get a 64-bit result, cast the left-hand operand first, e.g. `.as<u8>`)
#
meta:
  id: expr_shift_distance_eq_width_u4
seq:
  - id: u4_val
    type: u4le
instances:
  value:
    value: u4_val << 32
