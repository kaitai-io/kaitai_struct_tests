# repeat_expr_repeat-until.ksy: /seq/0/repeat:
# 	error: `repeat-until` requires either a `repeat: until` or absence of a `repeat` key
#
meta:
  id: repeat_expr_repeat_until
seq:
  - id: foo
    type: u1
    repeat: expr
    repeat-until: 'true'
