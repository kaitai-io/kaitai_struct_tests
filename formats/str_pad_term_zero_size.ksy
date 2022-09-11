# see "bytes_pad_term_zero_size"
meta:
  id: str_pad_term_zero_size
  encoding: UTF-8
seq:
  - id: str_pad
    type: str
    size: 0
    pad-right: 0x40
  - id: str_term
    type: str
    size: 0
    terminator: 0x40
  - id: str_term_and_pad
    type: str
    size: 0
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_include
    type: str
    size: 0
    terminator: 0x40
    include: true
