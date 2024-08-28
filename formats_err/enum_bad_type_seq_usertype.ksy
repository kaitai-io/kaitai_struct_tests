# enum_bad_type_seq_usertype.ksy: /seq/0/enum:
# 	error: tried to resolve non-integer UserTypeInstream(List(dummy),None,List()) to enum
#
meta:
  id: enum_bad_type_seq_usertype
seq:
  - id: foo
    type: dummy
    enum: animal
enums:
  animal:
    1: cat
    2: dog
types:
  dummy: {}
