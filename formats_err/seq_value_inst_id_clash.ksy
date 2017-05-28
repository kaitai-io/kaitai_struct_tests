# /instances/bar: duplicate attribute ID 'bar', previously defined at /seq/1
meta:
  id: seq_value_inst_id_clash
seq:
  - id: foo
    type: u1
  - id: bar
    type: u1
instances:
  baz:
    value: 0
  bar:
    value: "'blah'"
