meta:
  id: zlib_surrounded
seq:
  - id: pre
    size: 4
  - id: zlib
    size: 12
    process: zlib
    type: inflated
  - id: post
    size: 4
types:
  inflated:
    seq:
      - id: inflated
        type: s4le
