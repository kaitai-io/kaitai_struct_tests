# /seq/0/repeat-until: invalid type: expected boolean, got Int1Type(true)
meta:
  id: attr_invalid_repeat_until
seq:
  - id: foo
    type: u1
    repeat: until
    repeat-until: 5
