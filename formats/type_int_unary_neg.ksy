meta:
  id: type_int_unary_neg
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
  - id: u8_val
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
  neg_s1_val:
    value: -s1_val
  neg_s2_val:
    value: -s2_val
  neg_s4_val:
    value: -s4_val
  neg_s8_val:
    value: -s8_val

  neg_u1_val:
    value: -u1_val
  neg_u2_val:
    value: -u2_val
  neg_u4_val_msb_set:
    value: -u4_val_msb_set.as<s8>
  neg_u4_val_msb_clear:
    value: -u4_val_msb_clear.as<s8>
  neg_u8_val:
    value: -u8_val.as<s8>

  neg_b08_val:
    value: -b08_val
  neg_b16_val:
    value: -b16_val
  neg_b32_val:
    value: -b32_val.as<s8>
  neg_b64_val:
    value: -b64_val.as<s8>
  neg_b31_val:
    value: -b31_val
  neg_b63_val:
    value: -b63_val.as<s8>
