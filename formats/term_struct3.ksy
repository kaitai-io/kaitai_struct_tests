# Like "term_bytes3", but the byte array is wrapped in a user-defined type
meta:
  id: term_struct3
seq:
  - id: s1
    terminator: 0x7c
    consume: false
    type: bytes_wrapper
  - id: s2
    terminator: 0x40
    consume: false
    type: bytes_wrapper
  - id: s3
    terminator: 0x40
    type: bytes_wrapper
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
