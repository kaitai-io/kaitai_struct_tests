# expr_enum_unknown.ksy: /seq/1/if:
# 	error: unable to find enum 'unknown_enum', searching from expr_enum_unknown
#
meta:
  id: expr_enum_unknown
seq:
  - id: foo
    type: u1
    enum: animal
  - id: bar
    type: u1
    if: 'foo == unknown_enum::cat'
enums:
  animal:
    1: cat
    2: dog
