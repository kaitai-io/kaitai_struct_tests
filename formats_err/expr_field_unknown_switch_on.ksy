# expr_field_unknown_switch_on.ksy: /seq/0/type/switch-on:
# 	error: unable to access 'bar' in expr_field_unknown_switch_on context
#
meta:
  id: expr_field_unknown_switch_on
seq:
  - id: foo
    type:
      switch-on: bar
      cases:
        42: s1
