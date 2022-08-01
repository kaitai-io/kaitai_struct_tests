# params_dup_id.ksy: /params/2:
# 	error: duplicate attribute ID 'foo', previously defined at /params/0
#
meta:
  id: params_dup_id
params:
  - id: foo
    type: u1
  - id: bar
    type: u1
  - id: foo
    type: bytes
