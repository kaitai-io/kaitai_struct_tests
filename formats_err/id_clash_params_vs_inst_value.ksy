# id_clash_params_vs_inst_value.ksy: /instances/bar:
# 	error: duplicate attribute ID 'bar', previously defined at /params/1/id
#
meta:
  id: id_clash_params_vs_inst_value
params:
  - id: foo
    type: s1
  - id: bar
    type: bool
instances:
  baz:
    value: 0
  bar:
    value: "'blah'"
