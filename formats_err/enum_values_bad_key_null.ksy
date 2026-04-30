# enum_values_bad_key_null.ksy: /enums/animal/values:
# 	error: expected int, got null
#
meta:
  id: enum_values_bad_key_null
enums:
  animal:
    type: u1
    values:
      1: cat
      2: dog
      null: chicken
