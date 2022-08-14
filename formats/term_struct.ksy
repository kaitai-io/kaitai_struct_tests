# Like "term_bytes", but the byte array is wrapped in a user-defined type
meta:
  id: term_struct
seq:
  - id: s1
    terminator: 0x7c
    type: bytes_wrapper
  - id: s2
    terminator: 0x7c
    consume: false
    type: bytes_wrapper
  - id: s3
    terminator: 0x40
    include: true
    type: bytes_wrapper
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
