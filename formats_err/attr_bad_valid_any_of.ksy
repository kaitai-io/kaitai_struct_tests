# attr_bad_valid_any_of.ksy: /seq/0/valid/any-of/2:
# 	error: can't compare Int1Type(false) and CalcBooleanType
#
meta:
  id: attr_bad_valid_any_of
seq:
  - id: foo
    type: u1
    valid:
      any-of:
        - 0
        - 1
        - true
        - 3
