# expr_compare_enum2.ksy: /seq/1/if:
# 	error: can't compare EnumType(List(animal),Int1Type(false)) and Int1Type(true)
#
meta:
  id: expr_compare_enum2
seq:
  - id: foo
    type: u1
    enum: animal
  - id: bar
    type: u1
    if: foo == 2
enums:
  animal:
    1: cat
    2: dog
