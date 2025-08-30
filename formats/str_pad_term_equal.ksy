# Tests "pad-right" and "terminator" functionality in fixed-length strings ("str" with "size")
# with `terminator` == `pad-right`
meta:
  id: str_pad_term_equal
  encoding: UTF-8
seq:
  - id: s1
    type: str
    size: 20
    terminator: 0x40
    pad-right: 0x40
  - id: s2
    type: str
    size: 20
    terminator: 0x40
    include: true
    pad-right: 0x2b
  - id: s3
    type: str
    size: 20
    terminator: 0x2b
    pad-right: 0x2b
  - id: s4
    type: str
    size: 20
    terminator: 0x2e
    include: true
    pad-right: 0x2e
