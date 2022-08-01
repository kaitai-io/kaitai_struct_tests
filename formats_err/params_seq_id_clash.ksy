# params_seq_id_clash.ksy: /seq/0:
# 	error: duplicate attribute ID 'foo', previously defined at /params/1
#
meta:
  id: params_seq_id_clash
params:
  - id: baz
    type: bool
  - id: foo
    type: s1
seq:
  - id: foo
    type: u1
  - id: bar
    type: u1
