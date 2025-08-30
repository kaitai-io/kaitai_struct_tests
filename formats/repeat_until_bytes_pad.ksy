# see "repeat_n_bytes_pad"
meta:
  id: repeat_until_bytes_pad
seq:
  - id: records
    size: 5
    pad-right: 0xaa
    repeat: until
    repeat-until: _.length == 5
