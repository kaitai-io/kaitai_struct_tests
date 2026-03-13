meta:
  id: bits_byte_aligned
seq:
  - id: one
    type: b6
  # skips 2 bits
  - id: byte_1
    type: u1
  - id: two
    type: b3
  - id: three
    type: b1
  # skips 4 bits
  - id: byte_2
    size: 1
  - id: four
    type: b14
  # skips 2 bits
  - id: byte_3
    size: 3
    type: foo
  - id: full_byte
    type: b8
  - id: byte_4
    type: u1
  - id: five
    type: b22
  - id: bytes_term
    terminator: 0x45
    include: true
  - id: six
    type: b8
types:
  foo:
    seq:
      - id: inner
        type: b19
