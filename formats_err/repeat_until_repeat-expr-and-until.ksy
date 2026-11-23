# repeat_until_repeat-expr-and-until.ksy: /seq/0/repeat:
# 	error: either `repeat: eos`, or `repeat-expr`, or `repeat-until` must be specified
#
meta:
  id: repeat_until_repeat_expr_and_until
seq:
  - id: foo
    type: u1
    repeat: until
    repeat-expr: 42
    repeat-until: 'true'
