# id_dup_params.ksy: /params/2/id:
# 	error: duplicate attribute ID 'foo', previously defined at /params/0/id
#
meta:
  id: id_dup_params
params:
  - id: foo
    type: u1
  - id: bar
    type: u1
  - id: foo
    type: bytes
