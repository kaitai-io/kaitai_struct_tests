# params_parse_inst_id_clash.ksy: /instances/foo: error: duplicate attribute ID 'foo', previously defined at /params/0
meta:
  id: params_parse_inst_id_clash
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
