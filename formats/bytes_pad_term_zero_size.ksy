# Like "bytes_pad_term", but all `size`s are set to 0 (to make sure we don't
# fail in this case)
meta:
  id: bytes_pad_term_zero_size
seq:
  - id: str_pad
    size: 0
    pad-right: 0x40
  - id: str_term
    size: 0
    terminator: 0x40
  - id: str_term_and_pad
    size: 0
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_include
    size: 0
    terminator: 0x40
    include: true
