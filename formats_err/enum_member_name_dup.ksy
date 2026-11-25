# enum_member_name_dup.ksy: /enums/animal/values/101:
# 	error: duplicate enum member ID: 'cat', previously defined at /enums/animal/values/1
#
meta:
  id: enum_member_name_dup
enums:
  animal:
    type: u1
    values:
      1: cat
      2: dog
      101: cat
