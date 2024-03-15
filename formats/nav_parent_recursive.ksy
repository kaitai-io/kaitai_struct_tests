# Tests that `_parent` is passed to recursive invocations of the top-level type
#
# See https://github.com/kaitai-io/kaitai_struct/issues/1089
meta:
  id: nav_parent_recursive
seq:
  - id: value
    type: u1
  - id: next
    type: nav_parent_recursive
    if: value == 0xff
instances:
  parent_value:
    value: _parent.as<nav_parent_recursive>.value
    if: value != 0xff
