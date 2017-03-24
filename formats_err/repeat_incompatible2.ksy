# /seq/0/repeat: `repeat: expr` requires a `repeat-expr` expression
meta:
  id: repeat_incompatible2
seq:
  - id: foo
    type: u1
    repeat: expr
    repeat-until: 'true'
