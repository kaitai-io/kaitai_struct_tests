# Similar to "instance_in_repeat_expr", but the instance is used in `repeat-until`
meta:
  id: instance_in_repeat_until
  endian: le
seq:
  - id: entries
    type: s2
    repeat: until
    repeat-until: _ == until_val # NB: this is first evaluated after reading `entries[0]`
instances:
  until_val:
    pos: _io.pos + 12 # _io.pos is 2 when first invoked, so this should be at byte 14
    type: s2
