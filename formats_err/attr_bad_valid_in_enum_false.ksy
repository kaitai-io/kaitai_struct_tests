# attr_bad_valid_in_enum_false.ksy: /seq/0/valid/in-enum:
# 	error: only `true` is supported as value, got `false` (if you don't want any validation, omit the `valid` key)
#
meta:
  id: attr_bad_valid_in_enum_false
seq:
  - id: foo
    type: u1
    enum: animal
    valid:
      in-enum: false
enums:
  animal:
    1: cat
    2: dog
