# enum_underlying_type_mismatch.ksy: /seq/0/enum:
# 	error: unable to convert type `s4` to enum `animal` with underlying type `u4`
#
meta:
  id: enum_underlying_type_mismatch
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
