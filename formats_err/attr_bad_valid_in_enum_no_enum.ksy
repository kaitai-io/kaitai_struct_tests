# attr_bad_valid_in_enum_no_enum.ksy: /seq/0/valid/in-enum:
# 	error: `valid/in-enum` requires the `enum` key
#
meta:
  id: attr_bad_valid_in_enum_no_enum
seq:
  - id: foo
    type: u1
    valid:
      in-enum: true
