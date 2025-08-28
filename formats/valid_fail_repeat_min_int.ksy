meta:
  id: valid_fail_repeat_min_int
seq:
  - id: foo
    type: s1
    valid:
      min: 0 # there is actually -24 (0xe8) in the file
    repeat: eos
