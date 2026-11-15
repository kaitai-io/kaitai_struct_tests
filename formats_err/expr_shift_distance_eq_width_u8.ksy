# expr_shift_distance_eq_width_u8.ksy: /instances/value/value:
# 	error: shift distance 64 is out of range for `>>` on u8: the result type is u8, so the distance must be in 0..63
#
meta:
  id: expr_shift_distance_eq_width_u8
seq:
  - id: u8_val
    type: u8le
instances:
  value:
    value: u8_val >> 64
