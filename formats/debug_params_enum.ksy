meta:
  id: debug_params_enum
  ks-debug: true
enums:
  animal:
    4: dog
    7: cat
    12: chicken
seq:
  - id: one
    type: u1
    enum: animal
  - id: invoke_with_param
    type: with_param(one)
types:
  with_param:
    params:
      - id: enumerated_one
        type: u1
        enum: animal
