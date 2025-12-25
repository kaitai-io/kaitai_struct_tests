# repeat_until_repeat-expr.ksy: /seq/0/repeat:
# 	error: `repeat-expr` requires either a `repeat: expr` or absence of a `repeat` key
#
meta:
  id: repeat_until_repeat_expr
seq:
  - id: foo
    type: u1
    repeat: until
    repeat-expr: 42
