meta:
  id: expr_enum_to_s
seq:
  - id: one
    type: u4le
    enum: animal
instances:
  one_as_string:
    value: |
      "one=" + one.to_s
enums:
  animal:
    4: dog
    7: cat
    12: chicken
