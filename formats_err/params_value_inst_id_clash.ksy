# params_value_inst_id_clash.ksy: /instances/bar: error: duplicate attribute ID 'bar', previously defined at /params/1
meta:
  id: params_value_inst_id_clash
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
