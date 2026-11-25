# Ternary operator (`?:`) with integer branches as an operand of other
# operations. The result type of the ternary matters here, because only
# the result of the other operation is converted to the type of the value
# instance. The last case has a binary operation as a branch instead.
meta:
  id: type_int_ternary_operand
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

  # Signed result types with an unsigned branch (a `u4` literal, `b31_val`):
  # `<` must be a signed comparison. `cond` selects the negative branch.
  lt_s8_lit_u4:
    value: '(cond ? s8_val : 3_000_000_000) < 0'
  lt_s4_b31:
    value: '(cond ? s4_val : b31_val) < 0'

  # Operations whose results don't fit into the type of either branch
  mul_u1_u2:
    # The ternary is `u2`, but the multiplication is done in `s4`
    value: '(cond ? u1_val : u2_val) * 1000'
  neg_s2_u1:
    # The ternary is `s4` (not `s2`), just like `s2_val + u1_val`. `s2_val`
    # is `-0x8000`, so its negation doesn't fit into `s2`.
    value: '-(cond ? s2_val : u1_val)'

  # Ternaries with only literal branches: in Zig, literals have the
  # compile-time-only type `comptime_int`, which a ternary with a runtime
  # condition can't have, so the literals must be converted to a
  # fixed-width type (e.g. `@as(i8, 1)`)
  add_lit_lit_u1:
    # The ternary is `s1`, but the addition is done in `s4`
    value: '(cond ? 1 : 2) + u1_val'
  eq_lit_lit_s1:
    value: '(cond ? 1 : 2) == s1_val'

  # A binary operation as a branch
  add_u8_lit_u8:
    # The ternary is `u8`, like both branches
    value: 'cond ? u8_val_msb_set + 1 : u8_val_msb_set'
