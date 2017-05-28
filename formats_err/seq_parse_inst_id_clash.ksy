# /instances/foo: duplicate attribute ID 'foo', previously defined at /seq/0
meta:
  id: seq_parse_inst_id_clash
seq:
  - id: foo
    type: u1
  - id: bar
    type: u1
instances:
  baz:
    value: 0
  foo:
    pos: 0
    type: u1
