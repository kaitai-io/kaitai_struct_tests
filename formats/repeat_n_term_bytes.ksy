# see also "repeat_until_term_bytes"
meta:
  id: repeat_n_term_bytes
seq:
  - id: records1
    terminator: 0xaa
    repeat: expr
    repeat-expr: 2
  - id: records2
    terminator: 0xaa
    include: true
    repeat: expr
    repeat-expr: 2
  - id: records3
    terminator: 0x55
    consume: false
    repeat: expr
    repeat-expr: 2
