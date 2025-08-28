# expr_field_unknown_switch_params_call.ksy: /seq/0/type/cases/Bool(true):
# 	error: unable to access 'bar' in expr_field_unknown_switch_params_call context
#
meta:
  id: expr_field_unknown_switch_params_call
seq:
  - id: foo
    type:
      switch-on: true
      cases:
        true: param_type(42, bar)
types:
  param_type:
    params:
      - id: len
        type: u4
      - id: has_trailer
        type: bool
