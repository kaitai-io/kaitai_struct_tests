# id_clash_seq_vs_inst_pos.ksy: /instances/foo:
# 	error: duplicate attribute ID 'foo', previously defined at /seq/0/id
#
meta:
  id: id_clash_seq_vs_inst_pos
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
