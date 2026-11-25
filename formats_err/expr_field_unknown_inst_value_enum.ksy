# expr_field_unknown_inst_value_enum.ksy: /instances/foo/value:
# 	error: unable to access 'bar' in expr_field_unknown_inst_value_enum context
#
meta:
  id: expr_field_unknown_inst_value_enum
instances:
  foo:
    value: bar
    enum: fruit
enums:
  fruit:
    type: u1
    values:
      1: apple
      2: banana
