# expr_enum_member_unknown.ksy: /instances/not_an_animal/value:
# 	error: unable to find enum member 'animal::hat' (enum 'animal' defined at /enums/animal)
#
meta:
  id: expr_enum_member_unknown
instances:
  not_an_animal:
    value: animal::hat
enums:
  animal:
    1: cat
    2: dog
