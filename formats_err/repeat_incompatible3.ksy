# /seq/0/repeat-expr: unknown key found, expected: consume, doc, eos-error, id, if, include, repeat, repeat-until, terminator, type
meta:
  id: repeat_incompatible3
seq:
  - id: foo
    type: u1
    repeat: until
    repeat-until: 1 == 1
    repeat-expr: 42
