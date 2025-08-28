meta:
  id: str_pad_term_utf16
  encoding: UTF-16LE
seq:
  - id: str_term
    size: 10
    type: strz
  - id: str_term_include
    size: 10
    type: strz
    include: true
  - id: str_term_and_pad
    size: 9
    type: strz
    pad-right: 0x2b
