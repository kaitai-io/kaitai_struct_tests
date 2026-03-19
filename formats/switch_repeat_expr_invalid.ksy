# https://github.com/kaitai-io/kaitai_struct/issues/494
meta:
  id: switch_repeat_expr_invalid
  endian: le
seq:
  - id: codes
    type: u1
    repeat: expr
    repeat-expr: 3
  - id: body
    size: 4
    repeat: expr
    repeat-expr: 3
    type:
      switch-on: codes[_index]
      cases:
        0x01: one
        0x02: two
types:
  one:
    seq:
      - id: first
        size-eos: true
  two:
    seq:
      - id: second
        size-eos: true
