# Like "process_repeat_usertype", but with a dynamic `key` parameter of the built-in
# `process: xor(key)` that depends on the current stream state. This means that the `key`
# expression must be always evaluated just before parsing the 5-byte array backing each
# `blocks[i]` item - this is natural for parsing, but not for serialization.
#
# To make things worse for serialization, we also parse another byte in each `blocks[i]`
# substream outside of the `block` type with positional instances using `io` key. So that
# even if the serializing code converted the `blocks[i]` substream to a byte array and
# unprocessed it with `xor` right after leaving the `block` type, it wouldn't help and it
# would now be forced to use `xor` again (but still with the `key` process argument
# correctly evaluated in the context right before each `blocks[i]`), because contents of
# `blocks[i]` substreams have changed.
meta:
  id: process_repeat_usertype_dynarg_xor
  endian: le
seq:
  - id: blocks
    size: 5
    process: xor(0x9b ^ (_index << 4 | _io.pos))
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
