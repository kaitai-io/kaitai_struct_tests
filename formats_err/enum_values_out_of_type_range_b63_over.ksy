# enum_values_out_of_type_range_b63_over.ksy: /enums/animal/values/9223372036854775808:
# 	error: integer constant 9223372036854775808 is out of range 0..9223372036854775807 of the enum's underlying type `b63`
#
meta:
  id: enum_values_out_of_type_range_b63_over
enums:
  animal:
    type: b63
    values:
      0: low
      0x7fff_ffff_ffff_ffff: high
      0x8000_0000_0000_0000: over
