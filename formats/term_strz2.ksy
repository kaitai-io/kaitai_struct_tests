# Like "term_strz", but the order of fields is `include: true` and
# `consume: false` (in "term_strz" it's the other way around) - this may catch
# other bugs that passed "term_strz" just by luck.
meta:
  id: term_strz2
  endian: le
seq:
  - id: s1
    type: str
    encoding: UTF-8
    terminator: 0x7c
  - id: s2
    type: str
    encoding: UTF-8
    terminator: 0x7c
    include: true
  - id: s3
    type: str
    encoding: UTF-8
    terminator: 0x40
    consume: false
