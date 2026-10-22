# enum_values_out_of_type_range_u8_under.ksy: /enums/animal/values/-1:
# 	error: integer constant -1 is out of range 0..18446744073709551615 of the enum's underlying type `u8`
#
meta:
  id: enum_values_out_of_type_range_u8_under
enums:
  animal:
    type: u8
    values:
      0: low
      0xffff_ffff_ffff_ffff: high
      -1: under
