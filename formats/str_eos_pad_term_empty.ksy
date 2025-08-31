# Same as "str_eos_pad_term", but used with different input file that is
# meant to test fully empty strings
meta:
  id: str_eos_pad_term_empty
  encoding: UTF-8
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
        type: str
        size-eos: true
        pad-right: 0x40
  str_term_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x40
  str_term_and_pad_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x40
        pad-right: 0x2b
  str_term_include_type:
    seq:
      - id: value
        type: str
        size-eos: true
        terminator: 0x40
        include: true
