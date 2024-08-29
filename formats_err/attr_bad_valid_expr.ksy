# attr_bad_valid_expr.ksy: /seq/0/valid/expr:
# 	error: invalid type: expected boolean, got Int1Type(true)
#
meta:
  id: attr_bad_valid_expr
seq:
  - id: foo
    type: u1
    valid:
      expr: 5
