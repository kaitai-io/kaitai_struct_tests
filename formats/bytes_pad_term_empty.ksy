# Same as "bytes_pad_term", but used with different input file that is
# meant to test fully empty byte arrays (see "str_pad_term_empty")
meta:
  id: bytes_pad_term_empty
seq:
  - id: str_pad
    size: 20
    pad-right: 0x40
  - id: str_term
    size: 20
    terminator: 0x40
  - id: str_term_and_pad
    size: 20
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_include
    size: 20
    terminator: 0x40
    include: true
