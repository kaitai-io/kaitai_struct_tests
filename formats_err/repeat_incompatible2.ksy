# repeat_incompatible2.ksy: /seq/0/repeat:
# 	error: `repeat: expr` requires a `repeat-expr` expression
#
meta:
  id: repeat_incompatible2
seq:
  - id: foo
    type: u1
    repeat: expr
    repeat-until: 'true'
