meta:
  id: type_int_bin_promotion
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
  # Mixed integer types (the table in the "3. Binary operators" section of
  # https://github.com/kaitai-io/kaitai_struct/issues/1333): types narrower
  # than 32 bits are promoted to `s4`, except that `u1` and `u2` are promoted
  # to `u4` when combined with `u4`, and to `u8` when combined with `u8`. The
  # narrower operand is then converted to the type of the wider one.
  #
  # All valid combinations are tested, except `s4` with `s4` (no conversion
  # needed) and those in TypeIntBinArith (`u1` with `u1`, `u4` with `u4` and
  # `u8` with `u8`). The signed values are negative and the unsigned ones
  # have the most significant bit set, so that a wrong conversion gives a
  # different result.
  add_s1_s1:
    value: s1_val + s1_val
  add_s2_s1:
    value: s2_val + s1_val
  sub_s4_s1:
    # `-` instead of `+`, because `s4_val + s1_val` would overflow
    value: s4_val - s1_val
  add_s8_s1:
    value: s8_val + s1_val
  add_u1_s1:
    value: u1_val + s1_val
  add_u2_s1:
    value: u2_val + s1_val
  add_s2_s2:
    value: s2_val + s2_val
  sub_s4_s2:
    # `-` instead of `+`, because `s4_val + s2_val` would overflow
    value: s4_val - s2_val
  add_s8_s2:
    value: s8_val + s2_val
  add_u1_s2:
    value: u1_val + s2_val
  add_u2_s2:
    value: u2_val + s2_val
  add_s8_s4:
    # `s4_val` must stay negative when converted to `s8`
    value: s8_val + s4_val
  add_s4_u1:
    value: s4_val + u1_val
  add_s4_u2:
    value: s4_val + u2_val
  add_s8_s8:
    value: s8_val + s8_val
  add_s8_u1:
    value: s8_val + u1_val
  add_s8_u2:
    value: s8_val + u2_val
  sub_s8_u4:
    # `u4_val_msb_set` must stay positive when converted to `s8`
    value: s8_val - u4_val_msb_set
  sub_u4_s8:
    # The result is `s8` like in `sub_s8_u4`, but the unsigned operand is
    # on the left
    value: u4_val_msb_set - s8_val
  add_u2_u1:
    value: u2_val + u1_val
  add_u4_u1:
    # `u1_val` is promoted to `u4`, not to `s4` (which couldn't be mixed with
    # `u4`)
    value: u4_val_msb_set + u1_val
  add_u8_u1:
    value: u8_val_msb_set + u1_val
  add_u2_u2:
    value: u2_val + u2_val
  add_u4_u2:
    value: u4_val_msb_set + u2_val
  add_u8_u2:
    # `u2_val` is promoted to `u8`
    value: u8_val_msb_set + u2_val
  add_u8_u4:
    value: u8_val_msb_set + u4_val_msb_set

  # Integer literals: a literal takes the type of the other operand if its
  # value fits into it. This also applies if both operands are literals.
  add_u8_lit:
    # The literal takes the type of the other operand (`u8`), and the result
    # (`2**63`) doesn't fit into `s8`
    value: u8_val_msb_clear + 3
  sub_u8_lit:
    # The literal takes the type of the other operand (`u8`)
    value: u8_val_msb_set - 1
  add_lit_u4:
    # The literal is on the left, and it takes the type of the other operand
    # (`u4`)
    value: 1 + u4_val_msb_set
  sub_lit_u8:
    # The literal is on the left, its value only fits into `u8`, and the
    # result has the most significant bit set
    value: 0xffff_ffff_ffff_ffff - u8_val_msb_clear
  add_u4_lit_s8:
    # The literal doesn't fit into `u4`, so it keeps its own type `s8`, which
    # is also the result type
    value: u4_val_msb_set + 0x1_0000_0000
  sub_lit_lit:
    # `1` takes the type of `0xffff_ffff` (`u4`), so the result has the most
    # significant bit set
    value: 0xffff_ffff - 1

  # Bit-sized integers: in expressions, `b2`..`b31` follow the same rules as
  # `u1` and `u2` (see above), because all their values fit into `s4`.
  # `b32` follows the rules of `u4`, and `b33`..`b64` those of `u8`. Both
  # sides of the `b31`/`b32` boundary are tested. `b63` and `b64` are also
  # tested, because `b63` is promoted to `u8` like `b64`, even though all its
  # values would fit into `s8` too.
  add_b31_u4:
    # The operands fit into `s4`, but the result only fits into `u4`, which is
    # the result type
    value: b31_val + u4_val_msb_clear
  add_b32_u1:
    # `u1_val` is promoted to `u4`
    value: b32_val + u1_val
  sub_u4_b16:
    # `b16_val` is promoted to `u4`
    value: u4_val_msb_set - b16_val
  add_s8_b08:
    # `b08_val` is promoted to `s8`
    value: s8_val + b08_val
  add_s4_b31:
    # `b31_val` is promoted to `s4`, so it can be mixed with a signed operand
    value: s4_val + b31_val
  add_b64_lit:
    # The operands fit into `s8`, but the result (`2**63`) only fits into
    # `u8`, which is the result type
    value: b64_val + 3
  add_b63_lit:
    # `b63_val` is promoted to `u8`, so the result (`2**63`) isn't limited to
    # 63 bits
    value: b63_val + 4
