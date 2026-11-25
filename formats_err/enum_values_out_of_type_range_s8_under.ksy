# enum_values_out_of_type_range_s8_under.ksy: /enums/animal/values/-9223372036854775809:
# 	error: integer constant -9223372036854775809 is out of range -9223372036854775808..9223372036854775807 of the enum's underlying type `s8`
#
meta:
  id: enum_values_out_of_type_range_s8_under
enums:
  animal:
    type: s8
    values:
      -0x8000_0000_0000_0000: low
      0x7fff_ffff_ffff_ffff: high
      -0x8000_0000_0000_0001: under
