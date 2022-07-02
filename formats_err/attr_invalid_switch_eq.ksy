# attr_invalid_switch_eq.ksy: /seq/1/type/cases/Str(foo):
# 	error: can't compare Int1Type(false) and CalcStrType
#
meta:
  id: attr_invalid_switch_eq
seq:
  - id: foo
    type: u1
  - id: bar
    type:
      switch-on: foo
      cases:
        '"foo"': u4le
