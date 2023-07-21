# Tests _io.eof operation when bit-sized integers are involved
meta:
  id: expr_io_eof_bits
  bit-endian: be
  ks-debug: true
seq:
  - id: foo
    type: b20
  - id: bar
    type: b4
    if: not _io.eof
  # EOF reached here
  - id: baz
    type: b16
    if: not _io.eof
  - id: align
    size: 0
  - id: qux
    type: b16
    if: not _io.eof
