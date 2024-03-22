meta:
  id: instance_io_user_earlier
  endian: le
seq:
  - id: sized_a
    size: 6
    type: slot
  - id: sized_b
    size: 6
    type: slot
  - id: into_b
    type: foo
  - id: into_a_skipped
    type: foo
  - id: into_a
    type: foo
  - id: last_accessor
    type: baz
instances:
  a_mid:
    io: into_a.inst._io
    pos: 1
    type: u2
  b_mid:
    io: into_b.inst._io
    pos: 1
    type: u2

types:
  foo:
    seq:
      - id: indicator
        type: u1
      - id: bar
        type: u1
        if: inst._io.size != 0 and inst.content == 0x66
    instances:
      inst:
        io: 'indicator == 0xca ? _parent.sized_b._io : _parent.sized_a._io'
        pos: 1
        size: '_io.pos != 0x0e ? 4 : 0'
        type: slot
  slot:
    seq:
      - id: content
        type: u1
        if: _io.size != 0
    instances:
      last:
        pos: _io.size - 1
        type: u1
        if: _io.size != 0
  baz:
    seq:
      - id: v
        type: u1
        if: _parent.into_b.inst.last == 0x59
