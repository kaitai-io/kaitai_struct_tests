# expr_field_unknown.ksy: /seq/0/if: error: unable to access 'bar' in expr_field_unknown context
meta:
  id: expr_field_unknown
seq:
  - id: foo
    type: u1
    if: 'bar == 42'
