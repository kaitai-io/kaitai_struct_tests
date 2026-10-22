# enum_values_bad_key_str.ksy: /enums/animal/values:
# 	error: unable to parse `hello` as int
#
meta:
  id: enum_values_bad_key_str
enums:
  animal:
    type: u1
    values:
      1: cat
      2: dog
      hello: chicken
