meta:
  id: enum_to_i_invalid
seq:
  - id: pet_1
    type: u1
    enum: animal
  - id: pet_2
    type: u1
    enum: animal
enums:
  animal:
    0x66: dog
    0x7c: cat
instances:
  pet_2_i:
    value: pet_2.to_i
  pet_2_i_to_s:
    value: pet_2.to_i.to_s
  pet_2_mod:
    value: pet_2.to_i + 0x8000
  one_lt_two:
    value: pet_1.to_i < pet_2.to_i
  pet_2_eq_int_t:
    value: pet_2.to_i == 0x6f
  pet_2_eq_int_f:
    value: pet_2.to_i == 0x6e
