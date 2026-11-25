# enum_underlying_type_mismatch_s4_into_u4.ksy: /seq/0/enum:
# 	error: unable to convert type `s4` to enum `animal` with underlying type `u4` (range -2147483648..2147483647 of `s4` is not fully contained in range 0..4294967295 of `u4`)
#
meta:
  id: enum_underlying_type_mismatch_s4_into_u4
  endian: le
seq:
  - id: foo
    type: s4
    enum: animal
enums:
  animal:
    type: u4
    values:
      1: cat
      2: dog
