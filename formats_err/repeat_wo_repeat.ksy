# repeat_wo_repeat.ksy: /seq/0/repeat-until:
# 	error: unknown key found, expected: consume, doc, doc-ref, eos-error, id, if, include, repeat, terminator, type, valid
#
meta:
  id: repeat_wo_repeat
seq:
  - id: foo
    type: u1
    repeat-until: 'true'
