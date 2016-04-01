meta:
  id: position_in_seq
  endian: le
seq:
  - id: numbers
    type: u1
    repeat: expr
    repeat-expr: header.qty_numbers
types:
  header:
    seq:
      - id: qty_numbers
        type: u4
instances:
  header:
    position-abs: 0x10
    type: header
