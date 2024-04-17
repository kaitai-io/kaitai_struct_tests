# This is just a helper spec that calls "params_def_enum_imported", where the
# actual test case is (it has a top-level parameter, so it's easier to call it
# via this middleman .ksy spec rather than directly in the language-specific
# test specs).
meta:
  id: imports_params_def_enum_imported
  imports:
    - enum_import_seq
    - params_def_enum_imported
seq:
  - id: one
    type: enum_import_seq
  - id: two
    type: params_def_enum_imported(one.pet_1, one.pet_2)
