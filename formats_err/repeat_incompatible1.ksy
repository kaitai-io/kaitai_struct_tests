# repeat_incompatible1.ksy: /seq/0/repeat-until:
# 	error: unknown key found, expected: consume, doc, doc-ref, eos-error, id, if, include, repeat, terminator, type, valid
#
meta:
  id: repeat_incompatible1
seq:
  - id: foo
    type: u1
    repeat: eos
    repeat-until: 'true'
