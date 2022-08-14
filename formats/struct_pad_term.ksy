# Tests "pad-right" and "terminator" functionality in fixed-length structs
meta:
  id: struct_pad_term
seq:
  - id: str_pad
    size: 20
    pad-right: 0x40
    type: bytes_wrapper
  - id: str_term
    size: 20
    terminator: 0x40
    type: bytes_wrapper
  - id: str_term_and_pad
    size: 20
    terminator: 0x40
    pad-right: 0x2b
    type: bytes_wrapper
  - id: str_term_include
    size: 20
    terminator: 0x40
    include: true
    type: bytes_wrapper
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
