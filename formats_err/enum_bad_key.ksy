# enum_bad_key.ksy: /enums/animal/foo:
# 	error: unknown key found, expected: doc, doc-ref, type, values
#
meta:
  id: enum_bad_key
enums:
  animal:
    type: u1
    values:
      1: cat
      2: dog
    foo: bad_key
