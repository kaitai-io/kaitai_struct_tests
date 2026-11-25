# enum_name_bad.ksy: /enums/&aa:
# 	error: invalid enum ID: '&aa', expected /^[a-z][a-z0-9_]*$/
#
meta:
  id: enum_name_bad
enums:
  '&aa':
    type: u1
    values:
      1: cat
      2: dog
