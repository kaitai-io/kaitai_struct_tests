# Assert that enum id's can handle values in Long's range
meta:
  id: enum_long_range_u
  endian:  be

enums:
  constants:
    0: zero
    4294967295: int_max
    4294967296: int_over_max
    18446744073709551615: long_max

seq:
  - id: f1
    type: u8
    enum: constants
  - id: f2
    type: u8
    enum: constants
  - id: f3
    type: u8
    enum: constants
  - id: f4
    type: u8
    enum: constants
