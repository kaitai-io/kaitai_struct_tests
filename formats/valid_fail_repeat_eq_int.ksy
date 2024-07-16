meta:
  id: valid_fail_repeat_eq_int
  endian: be
seq:
  - id: foo
    type: u4
    valid: 0x12_34_56_78 # there is actually 0x00_12_34_56 in the file
    repeat: eos
