# id_clash_params_vs_inst_pos.ksy: /instances/foo:
# 	error: duplicate attribute ID 'foo', previously defined at /params/0/id
#
meta:
  id: id_clash_params_vs_inst_pos
params:
  - id: foo
    type: s1
  - id: bar
    type: bool
instances:
  baz:
    value: 0
  foo:
    pos: 0
    type: u1
