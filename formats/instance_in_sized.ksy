# Tests instances in substreams (mainly intended for writing, where it's a bit challenge to get everything right)
meta:
  id: instance_in_sized
  endian: le
seq:
  - id: cont
    size: 16
    type: wrapper
types:
  wrapper:
    seq:
      - id: seq_sized
        size: 4
        type: qux
      - id: seq_in_stream
        type: bar
    instances:
      inst_in_stream:
        pos: _io.pos + 3 # should be 8 (5 is where the `seq` ends)
        type: baz
      inst_sized:
        pos: _io.pos + 7 # should be 12 (5 is where the `seq` ends)
        size: 4
        type: qux

  bar:
    seq:
      - id: seq_f
        type: u1
    instances:
      inst:
        pos: 4 + 1
        size: 3

  baz:
    seq:
      - id: seq_f
        type: u1
    instances:
      inst:
        pos: 8 + 1
        size: 3

  qux:
    seq:
      - id: seq_f
        type: u1
        if: inst_invoked > 0
    instances:
      inst_invoked:
        pos: _io.pos + 1 # invoked from position 0 -> should be 2
        type: u1
      inst_unused_by_seq:
        pos: _io.pos + 1 # should be 2 (1 is where the `seq` ends)
        size: 2
