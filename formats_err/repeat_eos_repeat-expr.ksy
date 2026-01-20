# repeat_eos_repeat-expr.ksy: /seq/0/repeat:
# 	error: `repeat-expr` requires either a `repeat: expr` or absence of a `repeat` key
#
meta:
  id: repeat_eos_repeat_expr
seq:
  - id: foo
    type: u1
    repeat: eos
    repeat-expr: 42