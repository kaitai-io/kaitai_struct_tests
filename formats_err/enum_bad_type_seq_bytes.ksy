# enum_bad_type_seq_bytes.ksy: /seq/0/enum:
# 	error: tried to resolve non-integer BytesLimitType(IntNum(4),None,false,None,None) to enum
#
meta:
  id: enum_bad_type_seq_bytes
seq:
  - id: foo
    size: 4
    enum: animal
enums:
  animal:
    1: cat
    2: dog
types:
  dummy: {}
