# enum_values_out_of_type_range_s1_over.ksy: /enums/animal/values/128:
# 	error: integer constant 128 is out of range -128..127 of the enum's underlying type `s1`
#
meta:
  id: enum_values_out_of_type_range_s1_over
enums:
  animal:
    type: s1
    values:
      -0x80: low
      0x7f: high
      0x80: over
