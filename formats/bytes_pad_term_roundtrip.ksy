# Like "bytes_pad_term", but with added `pad-right`s to be able to test for an
# exact match with the original file after serialization
meta:
  id: bytes_pad_term_roundtrip
seq:
  - id: str_pad
    size: 20
    pad-right: 0x40
  - id: str_term
    size: 20
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_and_pad
    size: 20
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_include
    size: 20
    terminator: 0x40
    include: true
    pad-right: 0x2e
