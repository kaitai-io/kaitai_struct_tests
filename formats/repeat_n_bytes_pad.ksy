# see also "repeat_eos_bytes_pad" and "repeat_until_bytes_pad"
meta:
  id: repeat_n_bytes_pad
seq:
  - id: records
    size: 5
    pad-right: 0xaa
    repeat: expr
    repeat-expr: 3
