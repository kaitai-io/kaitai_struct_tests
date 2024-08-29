# attr_bad_valid_eq_short.ksy: /seq/0/valid:
# 	error: can't compare Int1Type(false) and CalcStrType
#
meta:
  id: attr_bad_valid_eq_short
seq:
  - id: foo
    type: u1
    valid: '"bar"'
