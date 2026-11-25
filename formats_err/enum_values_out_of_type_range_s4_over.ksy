# enum_values_out_of_type_range_s4_over.ksy: /enums/animal/values/2147483648:
# 	error: integer constant 2147483648 is out of range -2147483648..2147483647 of the enum's underlying type `s4`
#
meta:
  id: enum_values_out_of_type_range_s4_over
enums:
  animal:
    type: s4
    values:
      -0x8000_0000: low
      0x7fff_ffff: high
      0x8000_0000: over
