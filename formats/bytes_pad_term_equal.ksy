# Tests "pad-right" and "terminator" functionality in fixed-length byte arrays
# with `terminator` == `pad-right`
meta:
  id: bytes_pad_term_equal
seq:
  - id: s1
    size: 20
    terminator: 0x40
    pad-right: 0x40
  - id: s2
    size: 20
    terminator: 0x40
    include: true
    pad-right: 0x2b
  - id: s3
    size: 20
    terminator: 0x2b
    pad-right: 0x2b
  - id: s4
    size: 20
    terminator: 0x2e
    include: true
    pad-right: 0x2e
