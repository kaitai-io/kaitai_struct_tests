# expr_shift_distance_neg.ksy: /instances/value/value:
# 	error: shift distance -1 is out of range for `<<` on u4: the result type is u4, so the distance must be in 0..31
#
meta:
  id: expr_shift_distance_neg
seq:
  - id: u4_val
    type: u4le
instances:
  value:
    value: u4_val << -1
