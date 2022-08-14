# Tests "pad-right" and "terminator" functionality in fixed-length byte arrays with "process"
meta:
  id: process_bytes_pad_term
seq:
  - id: str_pad
    size: 20
    pad-right: 0x40
    process: xor(0x15)
  - id: str_term
    size: 20
    process: xor(0x15)
    terminator: 0x40
  - id: str_term_and_pad
    size: 20
    process: xor(0x15)
    terminator: 0x40
    pad-right: 0x2b
  - id: str_term_include
    size: 20
    process: xor(0x15)
    terminator: 0x40
    include: true
