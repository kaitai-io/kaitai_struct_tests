# This is a file that is imported in imports_enum_leaking.ksy
meta:
  id: enum_two
  # Note: missing import of `enum_one`
seq:
  - id: one
    type: u1
    enum: enum_one::one # Note: `enum_one` is not imported in this spec, so this should cause a compile error
instances:
  # Enum resolution in value instances significantly different from enum resolution in other attributes
  instance_one:
    value: 0
    enum: enum_one::one # Note: `enum_one` is not imported in this spec, so this should cause a compile error
enums:
  two:
    0: zero
