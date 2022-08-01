# attr_invalid_repeat_expr.ksy: /seq/0/repeat-expr:
# 	error: invalid type: expected integer, got CalcStrType
#
meta:
  id: attr_invalid_repeat_expr
seq:
  - id: foo
    type: u1
    repeat: expr
    repeat-expr: '"foo"'
