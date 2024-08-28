# id_clash_params_vs_seq.ksy: /seq/0/id:
# 	error: duplicate attribute ID 'foo', previously defined at /params/1/id
#
meta:
  id: id_clash_params_vs_seq
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
