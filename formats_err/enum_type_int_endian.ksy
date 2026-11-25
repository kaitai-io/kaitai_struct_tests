# enum_type_int_endian.ksy: /enums/animal/type:
# 	error: expected an integer type with no endianness (i.e. `uX` / `sX` / `bX`), got `u4le`
#
meta:
  id: enum_type_int_endian
enums:
  animal:
    type: u4le
    values:
      1: cat
      2: dog
