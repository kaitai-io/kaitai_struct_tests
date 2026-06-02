# This is a file that is imported in imports_type_leaking.ksy
meta:
  id: type_two
  # Note: missing import of `type_one`
seq:
  - id: one
    type: type_one # Note: `type_one` is not imported in this spec, so this should cause a compile error
