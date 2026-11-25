# Assert that enum id's can handle values in Long's range
meta:
  id: enum_long_range_s
  endian:  be

enums:
  constants:
    type: s8
    values:
      -9223372036854775808: long_min
      -2147483649: int_below_min
      -2147483648: int_min
      0: zero
      2147483647: int_max
      2147483648: int_over_max
      9223372036854775807: long_max

seq:
  - id: f1
    type: s8
    enum: constants
  - id: f2
    type: s8
    enum: constants
  - id: f3
    type: s8
    enum: constants
  - id: f4
    type: s8
    enum: constants
  - id: f5
    type: s8
    enum: constants
  - id: f6
    type: s8
    enum: constants
  - id: f7
    type: s8
    enum: constants

instances:
  f1_to_i:
    value: f1.to_i
  f2_to_i:
    value: f2.to_i
  f3_to_i:
    value: f3.to_i
  f4_to_i:
    value: f4.to_i
  f5_to_i:
    value: f5.to_i
  f6_to_i:
    value: f6.to_i
  f7_to_i:
    value: f7.to_i
