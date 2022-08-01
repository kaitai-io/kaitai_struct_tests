# repeat_incompatible3.ksy: /seq/0/repeat-expr:
# 	error: unknown key found, expected: consume, doc, doc-ref, eos-error, id, if, include, repeat, repeat-until, terminator, type, valid
#
meta:
  id: repeat_incompatible3
seq:
  - id: foo
    type: u1
    repeat: until
    repeat-until: 1 == 1
    repeat-expr: 42
