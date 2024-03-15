# Tests that `_root` is passed to recursive invocations of the top-level type
#
# See https://github.com/kaitai-io/kaitai_struct/issues/1089
meta:
  id: nav_root_recursive
seq:
  - id: value
    type: u1
  - id: next
    type: nav_root_recursive
    if: value == 0xff
instances:
  root_value:
    value: _root.value
