meta:
  id: type_int_bin_bitwise
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
  # Unsigned operands (all results have the most significant bit set)
  and_u8_lit:
    value: u8_val_msb_set & 0xffff_ffff_0000_0000
  or_u4_lit:
    value: u4_val_msb_clear | 0x8000_0000
  and_lit_u4:
    # The literal is on the left
    value: 0xffff_0000 & u4_val_msb_set
  xor_u4_u4:
    value: u4_val_msb_set ^ u4_val_msb_clear
  xor_u8_u8:
    value: u8_val_msb_set ^ u8_val_msb_clear

  # Mixing operand widths: the narrower operand is converted to the type of
  # the wider one
  or_u8_u4:
    value: u8_val_msb_set | u4_val_msb_set
  xor_u8_lit:
    # The literal takes the type of the other operand (`u8`)
    value: u8_val_msb_clear ^ 0xffff_ffff
  or_s8_s4:
    # `s4_val` must stay negative when converted to `s8`
    value: s8_val | s4_val
  xor_u2_u1:
    # Both operands are promoted to `s4`
    value: u2_val ^ u1_val

  # Mixing signed and unsigned operands
  and_s8_u4:
    # `u4_val_msb_set` must stay positive when converted to `s8`, so the
    # result is positive
    value: s8_val & u4_val_msb_set
  or_s4_u1:
    # `u1_val` is promoted to `s4`, so the operands can be mixed, and the
    # result is negative
    value: s4_val | u1_val
  xor_s8_lit:
    # The literal takes the type of the other operand (`s8`)
    value: s8_val ^ -1

  # Bit-sized integers
  and_b64_u8:
    # The result type is `u8`
    value: b64_val & u8_val_msb_set
  and_u8_b16:
    # `b16_val` is promoted to `u8`
    value: u8_val_msb_set & b16_val
  or_b31_u4:
    # `b31_val` is promoted to `u4`
    value: b31_val | u4_val_msb_set
