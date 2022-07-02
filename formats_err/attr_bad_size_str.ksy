# attr_bad_size_str.ksy: /seq/0/size:
# 	error: invalid type: expected integer, got CalcStrType
#
meta:
  id: attr_bad_size_str
seq:
  - id: foo
    type: str
    size: '"foo"'
    encoding: ASCII
