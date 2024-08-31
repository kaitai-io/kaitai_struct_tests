meta:
  id: valid_fail_repeat_max_int
seq:
  - id: foo
    type: u1
    valid:
      max: 0xfe # there is actually 0xff in the file
    repeat: eos
