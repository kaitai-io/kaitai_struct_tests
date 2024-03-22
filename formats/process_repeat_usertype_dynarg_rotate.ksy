# Like "process_repeat_usertype_dynarg_xor", but using `process: rol(n)` and
# `process: ror(n)`.
meta:
  id: process_repeat_usertype_dynarg_rotate
  endian: le
seq:
  - id: blocks_rol
    size: 3
    process: rol(_io.pos - 4 * _index)
    type: block
    repeat: expr
    repeat-expr: 2
  - id: blocks_ror
    size: 3
    process: ror((_io.pos - 6) - 4 * _index)
    type: block
    repeat: expr
    repeat-expr: 3
  - id: blocks_b
    type: blocks_b_wrapper

types:
  block:
    seq:
      - id: a
        type: u2
      # - id: b
      #   type: u1

  blocks_b_wrapper:
    seq:
      - id: dummy
        type: u1
    instances:
      blocks_rol_0_b:
        io: _parent.blocks_rol[0]._io
        pos: 2
        type: u1
      blocks_rol_1_b:
        io: _parent.blocks_rol[1]._io
        pos: 2
        type: u1

      blocks_ror_0_b:
        io: _parent.blocks_ror[0]._io
        pos: 2
        type: u1
      blocks_ror_1_b:
        io: _parent.blocks_ror[1]._io
        pos: 2
        type: u1
      blocks_ror_2_b:
        io: _parent.blocks_ror[2]._io
        pos: 2
        type: u1
