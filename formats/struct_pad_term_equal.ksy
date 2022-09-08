# Tests "pad-right" and "terminator" functionality in fixed-length structs
# with `terminator` == `pad-right`
meta:
  id: struct_pad_term_equal
seq:
  - id: s1
    size: 20
    terminator: 0x40
    pad-right: 0x40
    type: bytes_wrapper
  - id: s2
    size: 20
    terminator: 0x40
    include: true
    pad-right: 0x2b
    type: bytes_wrapper
  - id: s3
    size: 20
    terminator: 0x2b
    pad-right: 0x2b
    type: bytes_wrapper
  - id: s4
    size: 20
    terminator: 0x2e
    include: true
    pad-right: 0x2e
    type: bytes_wrapper
types:
  bytes_wrapper:
    seq:
      - id: value
        size-eos: true
