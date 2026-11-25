# enum_underlying_type_mismatch_s1_into_b7.ksy: /seq/0/enum:
# 	error: unable to convert type `s1` to enum `animal` with underlying type `b7` (range -128..127 of `s1` is not fully contained in range 0..127 of `b7`)
#
meta:
  id: enum_underlying_type_mismatch_s1_into_b7
  endian: le
seq:
  - id: foo
    type: s1
    enum: animal
enums:
  animal:
    type: b7
    values:
      1: cat
      2: dog
