meta:
  id: nan_leading
  endian: le
  encoding: UTF-8
seq:
  - id: afloat
    type: f4
    repeat: expr
    repeat-expr: 3
instances:
  afloat_max:
    value: afloat.max
  afloat_min:
    value: afloat.min
