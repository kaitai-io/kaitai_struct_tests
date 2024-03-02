# Check usage of a::b::c syntax where "a" is imported
meta:
  id: nested_types_import
  imports:
    - nested_types3
seq:
  - id: a_cc
    type: nested_types3::subtype_a::subtype_cc
  - id: a_c_d
    type: nested_types3::subtype_a::subtype_c::subtype_d
  - id: b
    type: nested_types3::subtype_b
