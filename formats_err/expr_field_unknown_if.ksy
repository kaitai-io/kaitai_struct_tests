# expr_field_unknown_if.ksy: /seq/0/if:
# 	error: unable to access 'bar' in expr_field_unknown_if context
#
meta:
  id: expr_field_unknown_if
seq:
  - id: foo
    type: u1
    if: 'bar == 42'
