meta:
  id: valid_fail_repeat_contents
seq:
  - id: foo
    contents: [0x12, 0x34, 0x56, 0x78] # there is actually [0x00, 0x12, 0x34, 0x56] in the file
    repeat: eos
