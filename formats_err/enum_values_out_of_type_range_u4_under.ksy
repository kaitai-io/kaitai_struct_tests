# enum_values_out_of_type_range_u4_under.ksy: /enums/animal/values/-1:
# 	error: integer constant -1 is out of range 0..4294967295 of the enum's underlying type `u4`
#
meta:
  id: enum_values_out_of_type_range_u4_under
enums:
  animal:
    type: u4
    values:
      0: low
      0xffff_ffff: high
      -1: under
