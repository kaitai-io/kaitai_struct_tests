# Tests "pad-right" and "terminator" functionality in fixed-length strings ("str" with "size")
# with `terminator` == `pad-right`
meta:
  id: str_eos_pad_term_equal
  encoding: UTF-8
seq:
  - id: s1
    size: 20
    type: s1_type
  - id: s2
    size: 20
    type: s2_type
  - id: s3
    size: 20
    type: s3_type
  - id: s4
    size: 20
    type: s4_type
types:
  s1_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x40
        pad-right: 0x40
  s2_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x40
        include: true
        pad-right: 0x2b
  s3_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x2b
        pad-right: 0x2b
  s4_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x2e
        include: true
        pad-right: 0x2e
