meta:
  id: expr_io_pos_bits
seq:
  - id: foo
    type: b3
  - id: bar
    type: b5
    if: _io.pos == 1
  - id: baz
    type: b1
    if: _io.pos == 1
  - id: qux
    type: b7
    if: _io.pos == 2
