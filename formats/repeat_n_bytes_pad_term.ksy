# see also "repeat_eos_bytes_pad_term" and "repeat_until_bytes_pad_term"
meta:
  id: repeat_n_bytes_pad_term
seq:
  - id: records
    size: 5
    terminator: 0x55
    include: true
    pad-right: 0xaa
    repeat: expr
    repeat-expr: 3
