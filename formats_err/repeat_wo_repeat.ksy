# /seq/0/repeat-until: unknown key found, expected: consume, doc, eos-error, id, if, include, repeat, terminator, type
meta:
  id: repeat_wo_repeat
seq:
  - id: foo
    type: u1
    repeat-until: 'true'
