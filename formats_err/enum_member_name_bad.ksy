# enum_member_name_bad.ksy: /enums/animal/101:
# 	error: invalid enum member ID: '101dogs', expected /^[a-z][a-z0-9_]*$/
#
meta:
  id: enum_member_name_bad
enums:
  animal:
    1: cat
    2: dog
    101: 101dogs
