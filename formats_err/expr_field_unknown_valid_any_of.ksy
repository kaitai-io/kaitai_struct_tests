# expr_field_unknown_valid_any_of.ksy: /seq/0/valid/any-of/1:
# 	error: unable to access 'bar' in expr_field_unknown_valid_any_of context
#
# expr_field_unknown_valid_any_of.ksy: /seq/0/valid/any-of/3:
# 	error: unable to access 'qux' in expr_field_unknown_valid_any_of context
#
meta:
  id: expr_field_unknown_valid_any_of
seq:
  - id: foo
    type: u1
    valid:
      any-of:
        - 0
        - bar
        - 2
        - qux
