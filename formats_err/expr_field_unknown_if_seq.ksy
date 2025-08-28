# expr_field_unknown_if_seq.ksy: /seq/0/if:
# 	error: unable to access 'bar' in expr_field_unknown_if_seq context
#
meta:
  id: expr_field_unknown_if_seq
seq:
  - id: foo
    type: u1
    if: bar
