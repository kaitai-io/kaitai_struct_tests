# expr_field_unknown_valid_range.ksy: /seq/0/valid/min:
# 	error: unable to access 'bar' in expr_field_unknown_valid_range context
#
# expr_field_unknown_valid_range.ksy: /seq/0/valid/max:
# 	error: unable to access 'qux' in expr_field_unknown_valid_range context
#
meta:
  id: expr_field_unknown_valid_range
seq:
  - id: foo
    type: u1
    valid:
      min: bar
      max: qux
