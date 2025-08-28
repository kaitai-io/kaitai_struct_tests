# Like "term_strz_utf16_v1", but tests `eos-error: false`
meta:
  id: term_strz_utf16_v4
  encoding: UTF-16LE
seq:
  - id: s1
    size: 6
    type: s1_type
  - id: skip_term1
    size: 2
  - id: s2
    size: 6
    type: s2_type
  - id: skip_term2
    size: 2
  - id: s3
    size: 6
    type: s3_type
types:
  s1_type:
    seq:
      - id: value
        type: strz
        eos-error: false

  s2_type:
    seq:
      - id: value
        type: strz
        consume: false
        eos-error: false

  s3_type:
    seq:
      - id: value
        type: strz
        include: true
        eos-error: false
