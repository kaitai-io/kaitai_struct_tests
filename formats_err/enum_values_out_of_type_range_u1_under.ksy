# enum_values_out_of_type_range_u1_under.ksy: /enums/animal/values/-1:
# 	error: integer constant -1 is out of range 0..255 of the enum's underlying type `u1`
#
meta:
  id: enum_values_out_of_type_range_u1_under
enums:
  animal:
    type: u1
    values:
      0: low
      0xff: high
      -1: under
