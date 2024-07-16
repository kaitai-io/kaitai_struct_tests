meta:
  id: valid_fail_repeat_anyof_int
seq:
  - id: foo
    type: u1
    valid: # there is actually 0xe8 in the file
      any-of:
        - 0
        - 1
        - 0x41
    repeat: eos
