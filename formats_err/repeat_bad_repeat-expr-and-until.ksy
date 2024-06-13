# repeat_bad_repeat-expr-and-until.ksy: /seq/0/repeat:
# 	error: expected eos / expr / until, got 'bad'
#
meta:
  id: repeat_bad_repeat_expr_and_until
seq:
  - id: foo
    type: u1
    repeat: bad
    repeat-expr: 42
    repeat-until: 'true'
