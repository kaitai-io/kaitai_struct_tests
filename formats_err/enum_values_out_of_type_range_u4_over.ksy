# enum_values_out_of_type_range_u4_over.ksy: /enums/animal/values/4294967296:
# 	error: integer constant 4294967296 is out of range 0..4294967295 of the enum's underlying type `u4`
#
meta:
  id: enum_values_out_of_type_range_u4_over
enums:
  animal:
    type: u4
    values:
      0: low
      0xffff_ffff: high
      0x1_0000_0000: over
