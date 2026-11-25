# enum_values_out_of_type_range_u8_over.ksy: /enums/animal/values/18446744073709551616:
# 	error: integer constant 18446744073709551616 is out of range 0..18446744073709551615 of the enum's underlying type `u8`
#
meta:
  id: enum_values_out_of_type_range_u8_over
enums:
  animal:
    type: u8
    values:
      0: low
      0xffff_ffff_ffff_ffff: high
      0x1_0000_0000_0000_0000: over
