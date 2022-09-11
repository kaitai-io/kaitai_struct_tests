# see "bytes_pad_term_roundtrip"
meta:
  id: str_pad_term_roundtrip
  encoding: UTF-8
seq:
  - id: str_pad
    type: str
    size: 20
    pad-right: 0x40
  - id: str_term
    type: str
    size: 20
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_and_pad
    type: str
    size: 20
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_include
    type: str
    size: 20
    terminator: 0x40
    include: true
    pad-right: 0x2e
