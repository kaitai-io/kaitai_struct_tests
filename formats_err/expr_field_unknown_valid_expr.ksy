# expr_field_unknown_valid_expr.ksy: /seq/0/valid/expr:
# 	error: unable to access 'bar' in expr_field_unknown_valid_expr context
#
meta:
  id: expr_field_unknown_valid_expr
seq:
  - id: foo
    type: u1
    valid:
      expr: bar
