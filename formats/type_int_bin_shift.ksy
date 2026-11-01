meta:
  id: type_int_bin_shift
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
  # Left shift: the bits shifted out of the result type are discarded
  shl_u1:
    # The result type is `s4`, so no bits are discarded
    value: u1_val << 8
  shl_u4:
    # The upper 4 bits are discarded
    value: u4_val_msb_set << 4
  shl_u8:
    # The result has the most significant bit set, so it would be negative as
    # `s8`
    value: u8_val_msb_clear << 1
  shl_s8:
    # The result is negative
    value: s8_val << 4
  shl_b32:
    # The upper 4 bits are discarded
    value: b32_val << 4
  shl_u1_as_u8:
    # The cast makes the result type `u8`, so the distance 40 is allowed
    value: u1_val.as<u8> << 40

  # Right shift: logical for unsigned types, arithmetic for signed types
  shr_u4:
    value: u4_val_msb_set >> 4
  shr_u8:
    # Only the upper 4 bits remain
    value: u8_val_msb_set >> 60
  shr_s4:
    value: s4_val >> 4
  shr_s8:
    value: s8_val >> 4
  shr_s1:
    # The result type is `s4`, and the result is negative
    value: s1_val >> 4
  shr_u2:
    # The result type is `s4`
    value: u2_val >> 4
  shr_b63:
    # `b63_val` is promoted to `u8`
    value: b63_val >> 1

  # The largest shift distances allowed: 31 for 32-bit result types, 63 for
  # 64-bit ones
  shl_u4_by_31:
    # The least significant bit becomes the most significant one, and all
    # other bits are discarded
    value: u4_val_msb_set << 31
  shl_u8_by_63:
    # The least significant bit becomes the most significant one, and all
    # other bits are discarded
    value: u8_val_msb_clear << 63
  shr_u4_by_31:
    # The most significant bit becomes the least significant one
    value: u4_val_msb_set >> 31
  shr_s4_by_31:
    # Only copies of the sign bit remain (`-1`)
    value: s4_val >> 31
  shr_u8_by_63:
    # The most significant bit becomes the least significant one
    value: u8_val_msb_set >> 63
  shr_s8_by_63:
    # Only copies of the sign bit remain (`-1`)
    value: s8_val >> 63

  # Non-constant shift distance (`u1_val & 7` is 5)
  shl_u4_var:
    value: u4_val_msb_set << (u1_val & 7)
  shr_u8_var:
    value: u8_val_msb_set >> (u1_val & 7)
  shr_s8_var:
    value: s8_val >> (u1_val & 7)
