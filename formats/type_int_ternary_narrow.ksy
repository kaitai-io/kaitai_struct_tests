# Ternary operator (`?:`) with `u1`/`u2` branches as an operand of `+`: the
# ternary is `u1` or `u2` (not `s4`), so it can be combined with `u4` and
# `u8` operands. These cases are in a test of their own, because a
# compile error in one expression fails every assert of a test.
meta:
  id: type_int_ternary_narrow
  endian: le
  bit-endian: le
seq:
  - id: s1_val
    type: s1
  - id: s2_val
    type: s2
  - id: s4_val
    type: s4
  - id: s8_val
    type: s8

  - id: u1_val
    type: u1
  - id: u2_val
    type: u2
  - id: u4_val_msb_set
    type: u4
  - id: u4_val_msb_clear
    type: u4
  - id: u8_val_msb_set
    type: u8
  - id: u8_val_msb_clear
    type: u8

  - id: b08_val
    type: b8
  - id: b16_val
    type: b16
  - id: b32_val
    type: b32
  - id: b64_val
    type: b64
  - id: b31_val
    type: b31
  - id: b63_val
    type: b63
instances:
  # The condition of all ternary operators below. They use this instance
  # instead of a literal `true`, because if the condition is known at compile
  # time, Zig doesn't analyze the branch that isn't selected, which would
  # hide type errors in it.
  cond:
    value: u1_val != 0

  add_u1_u2_u4:
    value: '(cond ? u1_val : u2_val) + u4_val_msb_set'
  add_u2_u1_u8:
    value: '(cond ? u2_val : u1_val) + u8_val_msb_set'
  add_u1_lit_u4:
    # `5` fits into `u1`, so the ternary is `u1`, not `s4` (which couldn't
    # be combined with `u4`)
    value: '(cond ? u1_val : 5) + u4_val_msb_set'
  add_b08_u2_u4:
    # `b08_val` follows the rules of `u1`, so the ternary is `u2`
    value: '(cond ? b08_val : u2_val) + u4_val_msb_set'
