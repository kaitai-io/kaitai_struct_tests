# attr_invalid_switch_inner.ksy: /seq/1/size:
# 	error: invalid type: expected integer, got CalcBooleanType
#
meta:
  id: attr_invalid_switch_inner
seq:
  - id: foo
    type: u1
  - id: bar
    size: 1 == 1
    type:
      switch-on: foo
      cases:
        42: dummy
types:
  dummy: {}
