# Like "term_bytes", but the order of fields is `include: true` and
# `consume: false` (in "term_bytes" it's the other way around) - this may catch
# other bugs that passed "term_bytes" just by luck.
meta:
  id: term_bytes2
seq:
  - id: s1
    terminator: 0x7c
  - id: s2
    terminator: 0x7c
    include: true
  - id: s3
    terminator: 0x40
    consume: false
