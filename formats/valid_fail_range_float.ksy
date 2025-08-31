meta:
  id: valid_fail_range_float
seq:
  - id: foo
    type: f4le
    valid:
      min: 0.25
      max: 0.375 # there is actually 0.5 in the file
