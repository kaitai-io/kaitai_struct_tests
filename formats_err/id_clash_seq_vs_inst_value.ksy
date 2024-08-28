# id_clash_seq_vs_inst_value.ksy: /instances/bar:
# 	error: duplicate attribute ID 'bar', previously defined at /seq/1/id
#
meta:
  id: id_clash_seq_vs_inst_value
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
