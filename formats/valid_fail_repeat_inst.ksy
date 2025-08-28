meta:
  id: valid_fail_repeat_inst
  endian: be
seq:
  - id: a
    size: 0
    if: inst.size == 0 # invoke instance
instances:
  inst:
    pos: 0
    type: u4
    valid: 0x12_34_56_78 # there is actually 0x00_12_34_56 in the file
    repeat: eos
