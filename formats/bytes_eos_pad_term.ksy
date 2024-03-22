# Tests "pad-right" and "terminator" functionality in fixed-length "size-eos: true" byte arrays
meta:
  id: bytes_eos_pad_term
seq:
  - id: str_pad
    size: 20
    type: str_pad_type
  - id: str_term
    size: 20
    type: str_term_type
  - id: str_term_and_pad
    size: 20
    type: str_term_and_pad_type
  - id: str_term_include
    size: 20
    type: str_term_include_type
types:
  str_pad_type:
    seq:
      - id: value
        size-eos: true
        pad-right: 0x40
  str_term_type:
    seq:
      - id: value
        size-eos: true
        terminator: 0x40
  str_term_and_pad_type:
    seq:
      - id: value
        size-eos: true
        terminator: 0x40
        pad-right: 0x2b
  str_term_include_type:
    seq:
      - id: value
        size-eos: true
        terminator: 0x40
        include: true
