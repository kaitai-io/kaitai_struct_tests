# enum_values_out_of_type_range_s1_under.ksy: /enums/animal/values/-129:
# 	error: integer constant -129 is out of range -128..127 of the enum's underlying type `s1`
#
meta:
  id: enum_values_out_of_type_range_s1_under
enums:
  animal:
    type: s1
    values:
      -0x80: low
      0x7f: high
      -0x81: under
