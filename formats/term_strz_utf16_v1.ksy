# Like "term_strz", but with multi-byte null terminator implied by `type: strz` with UTF-16
meta:
  id: term_strz_utf16_v1
  encoding: UTF-16LE
seq:
  - id: s1
    type: strz
  - id: s2
    type: strz
    consume: false
  - id: term
    type: u2le
  - id: s3
    type: strz
    include: true
