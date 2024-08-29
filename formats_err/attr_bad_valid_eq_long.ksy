# attr_bad_valid_eq_long.ksy: /seq/0/valid/eq:
# 	error: can't compare Int1Type(false) and CalcStrType
#
meta:
  id: attr_bad_valid_eq_long
seq:
  - id: foo
    type: u1
    valid:
      eq: '"bar"'
