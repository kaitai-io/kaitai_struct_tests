# Tests basic functionality of f-strings (formatted strings, interpolated
# strings): just interpolation of values into the strings, no fancy
# formatting.
meta:
  id: expr_fstring_0
  encoding: ASCII
seq:
  - id: seq_str
    type: str
    size: 5
  - id: seq_int
    type: u1
instances:
  empty:
    value: 'f""'
  literal:
    value: 'f"abc"'
  literal_with_escapes:
    value: |
      f"abc\n\tt"

  head_and_int_literal:
    value: |
      f"abc={123}"
  head_and_str_literal:
    value: |
      f"abc={'foo'}"
  head_and_int:
    value: 'f"abc={seq_int}"'
  head_and_str:
    value: 'f"abc={seq_str}"'
