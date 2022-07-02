# instance_pos_bad.ksy: /instances/foo/pos:
# 	error: invalid type: expected integer, got CalcStrType
#
meta:
  id: instance_pos_bad
instances:
  foo:
    pos: '"true"'
    type: u1
