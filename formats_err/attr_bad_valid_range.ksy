# attr_bad_valid_range.ksy: /seq/0/valid/min:
# 	error: can't compare Int1Type(false) and CalcBooleanType
#
# attr_bad_valid_range.ksy: /seq/0/valid/max:
# 	error: can't compare Int1Type(false) and CalcStrType
#
meta:
  id: attr_bad_valid_range
seq:
  - id: foo
    type: u1
    valid:
      min: true
      max: '"qux"'
