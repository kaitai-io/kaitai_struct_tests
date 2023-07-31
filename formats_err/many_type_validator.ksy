# many_type_validator.ksy: /seq/0/if:
# 	error: invalid type: expected boolean, got Int1Type(true)
#
# many_type_validator.ksy: /seq/2/if:
# 	error: can't use comparison operator Gt on enums
#
meta:
  id: many_type_validator
seq:
  - id: foo_int
    type: u1
    if: 5
  - id: foo_enum
    type: u1
    enum: animal
  - id: bar
    type: u1
    if: 'foo_enum > animal::cat'
enums:
  animal:
    1: cat
    2: dog
