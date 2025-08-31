# Like "repeat_n_term_bytes", but the byte array is wrapped in a user-defined type
meta:
  id: repeat_n_term_struct
seq:
  - id: records1
    terminator: 0xaa
    type: bytes_wrapper
    repeat: expr
    repeat-expr: 2
  - id: records2
    terminator: 0xaa
    include: true
    type: bytes_wrapper
    repeat: expr
    repeat-expr: 2
  - id: records3
    terminator: 0x55
    consume: false
    type: bytes_wrapper
    repeat: expr
    repeat-expr: 2
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
