# This is just a helper spec that calls "params_def_usertype_imported", where
# the actual test case is (it has a top-level parameter, so it's easier to call
# it via this middleman .ksy spec rather than directly in the language-specific
# test specs).
meta:
  id: imports_params_def_usertype_imported
  imports:
    - hello_world
    - params_def_usertype_imported
seq:
  - id: hw
    type: hello_world
  - id: two
    type: params_def_usertype_imported(hw)
