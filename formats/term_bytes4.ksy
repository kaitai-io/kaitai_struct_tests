# Like "term_bytes", but tests `eos-error: false`
meta:
  id: term_bytes4
seq:
  - id: s1
    size: 3
    type: s1_type
  - id: skip_term1
    type: u1
  - id: s2
    size: 3
    type: s2_type
  - id: skip_term2
    type: u1
  - id: s3
    size: 3
    type: s3_type
types:
  s1_type:
    seq:
      - id: value
        terminator: 0x7c
        eos-error: false
  s2_type:
    seq:
      - id: value
        terminator: 0x7c
        consume: false
        eos-error: false
  s3_type:
    seq:
      - id: value
        terminator: 0x40
        include: true
        eos-error: false
