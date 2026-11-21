# Tests division operation, both positive and negative
# See https://github.com/kaitai-io/kaitai_struct/issues/746
#  => the KS division operation `a / b` should do `floor(a / b)`
meta:
  id: expr_int_div
  endian: le
seq:
  - id: int_u
    type: u4
  - id: int_s
    type: s4
  - id: int_s8
    type: s8
instances:
  div_pos_const:
    value: 9837 / 13
  div_neg_const:
    value: -9837 / 13
  div_pos_seq:
    value: int_u / 13
  div_neg_seq:
    value: int_s / 13
  div_pos_s8_seq:
    # The result is odd and greater than `2**53`, so floating-point types such
    # as JavaScript's `number` can't represent it exactly (a `double` has a
    # 53-bit significand)
    value: int_s8 / 13
  div_neg_s8_seq:
    # The result is negative, and the division isn't exact, so rounding
    # toward zero would give a different result than rounding down
    value: -int_s8 / 13
