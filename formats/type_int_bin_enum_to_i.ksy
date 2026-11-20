# `.to_i` of an enum value gives an integer of the enum's underlying type,
# which must work like any other operand of that type in arithmetic
# operations
meta:
  id: type_int_bin_enum_to_i
  endian: le
seq:
  - id: u4_enum_val
    type: u4
    enum: u4_enum
  - id: u8_enum_val
    type: u8
    enum: u8_enum
  - id: s4_enum_val
    type: s4
    enum: s4_enum
  - id: u4_val
    type: u4
  - id: u1_val
    type: u1
  - id: s1_val
    type: s1
enums:
  u4_enum:
    type: u4
    values:
      0x8000_0000: msb_set
  u8_enum:
    type: u8
    values:
      0x1_0000_0000: over_32_bits
  s4_enum:
    type: s4
    values:
      -2: minus_two
instances:
  add_u4_enum_u4:
    value: u4_enum_val.to_i + u4_val
  add_u8_enum_u1:
    # `u1_val` is promoted to `u8`
    value: u8_enum_val.to_i + u1_val
  add_s4_enum_s1:
    # `s1_val` is promoted to `s4`
    value: s4_enum_val.to_i + s1_val
