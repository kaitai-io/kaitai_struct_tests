meta:
  id: valid_fail_in_enum
  endian: le
seq:
  - id: foo
    type: u4
    enum: animal
    valid: # there is actually 7 in the file
      in-enum: true
enums:
  animal:
    type: u4
    values:
      4: dog
      12: chicken
