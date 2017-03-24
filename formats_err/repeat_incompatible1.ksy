# /seq/0/repeat-until: unknown key found, expected: consume, doc, eos-error, id, if, include, repeat, terminator, type
meta:
  id: repeat_incompatible1
seq:
  - id: foo
    type: u1
    repeat: eos
    repeat-until: 'true'
