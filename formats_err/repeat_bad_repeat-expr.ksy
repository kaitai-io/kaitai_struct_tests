# repeat_bad_repeat-expr.ksy: /seq/0/repeat:
# 	error: expected eos / expr / until, got 'bad'
#
meta:
  id: repeat_bad_repeat_expr
seq:
  - id: foo
    type: u1
    repeat: bad
    repeat-expr: 42
