# Ternary operator (`?:`) with integer branches of different types: all
# valid combinations of two different integer types, plus branches with
# integer literals and with bit-sized integers, each in both branch
# orders. The result types follow the table in the "5. Ternary operator"
# section of https://github.com/kaitai-io/kaitai_struct/issues/1333.
meta:
  id: type_int_ternary_mixed
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

  # Mixed integer types: the result type is the same as for binary
  # arithmetic operations, except that if both branches are `s1`/`s2` or both
  # are `u1`/`u2`, the wider of the two types is the result type (instead of
  # `s4`). Both branches are converted to the result type.
  #
  # All valid combinations of two different types are tested, in the three
  # groups below. Each group lists its pairs in Bash brace expansion syntax.
  # Each pair is followed by the reverse order of its branches (e.g. `s1_s2`
  # is followed by `s2_s1`). `cond` is true, so each value is selected in one
  # of the two orders. The signed values are negative and the unsigned ones
  # have the most significant bit set, so that a wrong conversion gives a
  # different result.
  #
  # Signed with signed: `s1_{s2,s4,s8}`, `s2_{s4,s8}`, `s4_s8`
  s1_s2:
    # Both branches are signed and narrower than 32 bits, so the result type
    # is `s2`
    value: 'cond ? s1_val : s2_val'
  s2_s1:
    value: 'cond ? s2_val : s1_val'
  s1_s4:
    value: 'cond ? s1_val : s4_val'
  s4_s1:
    value: 'cond ? s4_val : s1_val'
  s1_s8:
    value: 'cond ? s1_val : s8_val'
  s8_s1:
    value: 'cond ? s8_val : s1_val'
  s2_s4:
    value: 'cond ? s2_val : s4_val'
  s4_s2:
    value: 'cond ? s4_val : s2_val'
  s2_s8:
    value: 'cond ? s2_val : s8_val'
  s8_s2:
    value: 'cond ? s8_val : s2_val'
  s4_s8:
    # `s4_val` must stay negative when converted to `s8`
    value: 'cond ? s4_val : s8_val'
  s8_s4:
    value: 'cond ? s8_val : s4_val'

  # Unsigned with unsigned: `u1_{u2,u4,u8}`, `u2_{u4,u8}`, `u4_u8`
  u1_u2:
    # Both branches are unsigned and narrower than 32 bits, so the result
    # type is `u2`
    value: 'cond ? u1_val : u2_val'
  u2_u1:
    value: 'cond ? u2_val : u1_val'
  u1_u4:
    value: 'cond ? u1_val : u4_val_msb_set'
  u4_u1:
    value: 'cond ? u4_val_msb_set : u1_val'
  u1_u8:
    value: 'cond ? u1_val : u8_val_msb_set'
  u8_u1:
    value: 'cond ? u8_val_msb_set : u1_val'
  u2_u4:
    value: 'cond ? u2_val : u4_val_msb_set'
  u4_u2:
    value: 'cond ? u4_val_msb_set : u2_val'
  u2_u8:
    value: 'cond ? u2_val : u8_val_msb_set'
  u8_u2:
    value: 'cond ? u8_val_msb_set : u2_val'
  u4_u8:
    value: 'cond ? u4_val_msb_set : u8_val_msb_set'
  u8_u4:
    value: 'cond ? u8_val_msb_set : u4_val_msb_set'

  # Signed with unsigned: `{s1,s2,s4}_{u1,u2}`, `s8_{u1,u2,u4}`. No signed
  # type can be mixed with `u8`, and only `s8` can be mixed with `u4`.
  s1_u1:
    # The branches have different signedness, so the result type is `s4`
    value: 'cond ? s1_val : u1_val'
  u1_s1:
    value: 'cond ? u1_val : s1_val'
  s1_u2:
    value: 'cond ? s1_val : u2_val'
  u2_s1:
    value: 'cond ? u2_val : s1_val'
  s2_u1:
    # All values of `u1` fit into `s2`, but the branches have different
    # signedness, so the result type is `s4`
    value: 'cond ? s2_val : u1_val'
  u1_s2:
    value: 'cond ? u1_val : s2_val'
  s2_u2:
    value: 'cond ? s2_val : u2_val'
  u2_s2:
    value: 'cond ? u2_val : s2_val'
  s4_u1:
    value: 'cond ? s4_val : u1_val'
  u1_s4:
    value: 'cond ? u1_val : s4_val'
  s4_u2:
    value: 'cond ? s4_val : u2_val'
  u2_s4:
    value: 'cond ? u2_val : s4_val'
  s8_u1:
    value: 'cond ? s8_val : u1_val'
  u1_s8:
    value: 'cond ? u1_val : s8_val'
  s8_u2:
    value: 'cond ? s8_val : u2_val'
  u2_s8:
    value: 'cond ? u2_val : s8_val'
  s8_u4:
    # `u4_val_msb_set` must stay positive when converted to `s8`
    value: 'cond ? s8_val : u4_val_msb_set'
  u4_s8:
    value: 'cond ? u4_val_msb_set : s8_val'

  # Integer literals: a literal takes the type of the other branch if its
  # value fits into it. Otherwise, the literal keeps its own type, which is
  # combined with the type of the other branch as above.
  s1_lit_u1:
    # `200` doesn't fit into `s1`, so it keeps its own type `u1`, and the
    # result type is `s4`
    value: 'cond ? s1_val : 200'
  lit_u1_s1:
    value: 'cond ? 200 : s1_val'
  u1_lit_s4:
    # `-1` doesn't fit into `u1`, so it keeps its own type `s4`, which is
    # also the result type
    value: 'cond ? u1_val : -1'
  lit_s4_u1:
    value: 'cond ? -1 : u1_val'
  s8_lit_u4:
    # The literal's own type is `u4`, but it takes the type of the other
    # branch (`s8`)
    value: 'cond ? s8_val : 3_000_000_000'
  lit_u4_s8:
    value: 'cond ? 3_000_000_000 : s8_val'
  lit_s4_lit_u1:
    # `200` fits into `s4` (the type of `-1`), which is the result type
    value: 'cond ? -1 : 200'
  lit_u1_lit_s4:
    value: 'cond ? 200 : -1'

  # Bit-sized integers: `b2`..`b8` follow the rules of `u1`, and `b9`..`b16`
  # those of `u2`. `b17`..`b31` are promoted as in binary arithmetic
  # operations: to `u4` when combined with `u4`, to `u8` when combined with
  # `u8`, and to `s4` otherwise. `b32` is promoted to `u4`, and `b33`..`b64`
  # to `u8`.
  b08_s1:
    # The result type is `s4`, as for `u1` with `s1`
    value: 'cond ? b08_val : s1_val'
  s1_b08:
    value: 'cond ? s1_val : b08_val'
  b16_u1:
    # The result type is `u2`, as for `u2` with `u1`
    value: 'cond ? b16_val : u1_val'
  u1_b16:
    value: 'cond ? u1_val : b16_val'
  b08_b16:
    # The result type is `u2`, as for `u1` with `u2`
    value: 'cond ? b08_val : b16_val'
  b16_b08:
    value: 'cond ? b16_val : b08_val'
  b31_s4:
    # `b31_val` is promoted to `s4`, so it can be mixed with a signed branch
    value: 'cond ? b31_val : s4_val'
  s4_b31:
    value: 'cond ? s4_val : b31_val'
  b31_u4:
    # `b31_val` is promoted to `u4`, not to `s4` (which couldn't be mixed
    # with `u4`)
    value: 'cond ? b31_val : u4_val_msb_set'
  u4_b31:
    value: 'cond ? u4_val_msb_set : b31_val'
  b32_s8:
    # `b32_val` is promoted to `u4`, and all values of `u4` fit into `s8`,
    # which is the result type
    value: 'cond ? b32_val : s8_val'
  s8_b32:
    value: 'cond ? s8_val : b32_val'
  b64_u8:
    # `b64_val` is promoted to `u8`
    value: 'cond ? b64_val : u8_val_msb_set'
  u8_b64:
    value: 'cond ? u8_val_msb_set : b64_val'
