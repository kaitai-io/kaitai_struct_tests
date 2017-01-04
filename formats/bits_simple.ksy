meta:
  id: bits_simple
seq:
  # byte 0
  - id: byte_1
    type: b8
  # byte 1
  - id: byte_2
    type: b8
  # byte 2 (8 bits = 1 + 3 + 4)
  - id: bits_a
    type: b1
  - id: bits_b
    type: b3
  - id: bits_c
    type: b4
  # byte 3-4-5 (24 bits = 10 + 3 + 11)
  - id: large_bits_1
    type: b10
  - id: spacer
    type: b3
  - id: large_bits_2
    type: b11
  # byte 6-7
  - id: normal_s2
    type: s2be
  # byte 8-9-10 (24 bits)
  - id: byte_8_9_10
    type: b24
