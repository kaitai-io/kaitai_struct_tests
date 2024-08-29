# expr_field_unknown_if_inst_pos.ksy: /instances/foo/if:
# 	error: unable to access 'bar' in expr_field_unknown_if_inst_pos context
#
meta:
  id: expr_field_unknown_if_inst_pos
instances:
  foo:
    pos: 0
    type: u1
    if: bar
