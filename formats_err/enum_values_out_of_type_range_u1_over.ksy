# enum_values_out_of_type_range_u1_over.ksy: /enums/animal/values/256:
# 	error: integer constant 256 is out of range 0..255 of the enum's underlying type `u1`
#
meta:
  id: enum_values_out_of_type_range_u1_over
enums:
  animal:
    type: u1
    values:
      0: low
      0xff: high
      0x100: over
