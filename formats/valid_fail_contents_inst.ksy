meta:
  id: valid_fail_contents_inst
seq:
  - id: a
    size: 0
    if: foo.size == 0 # invoke instance
instances:
  foo:
    pos: 0
    contents: [0x51, 0x41] # there is actually [0x50, 0x41] in the file
