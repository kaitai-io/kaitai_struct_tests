meta:
  id: valid_fail_range_bytes
seq:
  - id: foo
    size: 2
    valid:
      min: '[72, 64]'
      max: '[79, 126]' # there is actually [80, 65] in the file
