# Assert that enum id's can handle values in Int's range
meta:
  id: enum_int_range_s
  endian:  be

enums:
  constants:
    type: s4
    values:
      -2147483648: int_min
      0: zero
      2147483647: int_max

seq:
  - id: f1
    type: s4
    enum: constants
  - id: f2
    type: s4
    enum: constants
  - id: f3
    type: s4
    enum: constants

instances:
  f1_to_i:
    value: f1.to_i
  f2_to_i:
    value: f2.to_i
  f3_to_i:
    value: f3.to_i
