# enum_unquoted_null.ksy: /enums/foo/values/0:
# 	error: expected string or map, got null
#
meta:
  id: enum_unquoted_null
enums:
  foo:
    type: u1
    values:
      0: null
