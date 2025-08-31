# Like "process_repeat_usertype_dynarg_xor", but using a custom `process` with
# arguments.
meta:
  id: process_repeat_usertype_dynarg_custom
  endian: le
seq:
  - id: blocks
    size: 5
    process: 'my_custom_fx(_io.pos + 13 * _index, _io.pos % 2 == 0, _index == 1 ? [0x20, 0x30] : [0x40])'
    type: block
    repeat: expr
    repeat-expr: 2
  - id: blocks_b
    type: blocks_b_wrapper

types:
  block:
    seq:
      - id: a
        type: u4
      # - id: b
      #   type: u1

  blocks_b_wrapper:
    seq:
      - id: dummy
        type: u1
    instances:
      blocks_0_b:
        io: _parent.blocks[0]._io
        pos: 4
        type: u1
      blocks_1_b:
        io: _parent.blocks[1]._io
        pos: 4
        type: u1
