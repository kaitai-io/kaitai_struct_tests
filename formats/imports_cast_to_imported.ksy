# This is just a helper spec that calls "cast_to_imported", where the actual
# test case is (it has a top-level parameter, so it's easier to call it via this
# middleman .ksy spec rather than directly in the language-specific test specs).
meta:
  id: imports_cast_to_imported
  imports:
    - hello_world
    - cast_to_imported
seq:
  - id: hw
    type: hello_world
  - id: two
    type: cast_to_imported(hw)
