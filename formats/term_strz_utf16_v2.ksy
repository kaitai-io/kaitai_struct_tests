# Like "term_strz_utf16_v1", but the order of fields is `include: true` and
# `consume: false` (in "term_strz" it's the other way around) - this may catch
# other bugs that passed "term_strz_utf16_v1" just by luck.
meta:
  id: term_strz_utf16_v2
  encoding: UTF-16LE
seq:
  - id: s1
    type: strz
  - id: s2
    type: strz
    include: true
  - id: s3
    type: strz
    consume: false
