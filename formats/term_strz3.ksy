# Another variation of "term_strz" - this time with two `consume: false` fields and a 0-byte field
# (`s3`). See "term_bytes3" for more details.
meta:
  id: term_strz3
seq:
  - id: s1
    type: str
    encoding: UTF-8
    terminator: 0x7c
    consume: false
  - id: s2
    type: str
    encoding: UTF-8
    terminator: 0x40
    consume: false
  - id: s3
    type: str
    encoding: UTF-8
    terminator: 0x40
