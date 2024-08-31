# enum_bad_type_inst_value.ksy: /instances/foo/enum:
# 	error: tried to resolve non-integer CalcFloatType to enum
#
meta:
  id: enum_bad_type_inst_value
instances:
  foo:
    value: 1.5
    enum: animal
enums:
  animal:
    1: cat
    2: dog
