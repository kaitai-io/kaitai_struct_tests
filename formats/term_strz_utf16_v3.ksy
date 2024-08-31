# Another variation of "term_strz_utf16_v1" - this time with two `consume: false` fields and a
# 0-byte field (`s3`). See "term_bytes3" for more details.
meta:
  id: term_strz_utf16_v3
  encoding: UTF-16LE
seq:
  - id: s1
    type: strz
    consume: false
  - id: term
    type: u2le
  - id: s2
    type: strz
    consume: false
  - id: s3
    type: strz
