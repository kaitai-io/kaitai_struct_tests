# expr_shift_distance_over_31_u1.ksy: /instances/value/value:
# 	error: shift distance 40 is out of range for `<<` on u1: the result type is s4, so the distance must be in 0..31 (to get a 64-bit result, cast the left-hand operand first, e.g. `.as<u8>`)
#
meta:
  id: expr_shift_distance_over_31_u1
seq:
  - id: lsb
    type: u1
  - id: middle
    type: u4le
  - id: msb
    type: u1
instances:
  value:
    value: msb << 40 | middle << 8 | lsb
