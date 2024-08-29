# expr_field_unknown_if_inst_value.ksy: /instances/foo/if:
# 	error: unable to access 'bar' in expr_field_unknown_if_inst_value context
#
meta:
  id: expr_field_unknown_if_inst_value
instances:
  foo:
    value: 42
    if: bar
