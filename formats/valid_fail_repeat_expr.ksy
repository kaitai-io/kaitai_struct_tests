meta:
  id: valid_fail_repeat_expr
seq:
  - id: foo
    size: 4
    valid:
      # there is actually [0x00, 0x12, 0x34, 0x56] in the file
      # (this `valid/expr` asserts that this particular value does not occur)
      expr: _ != [0x00, 0x12, 0x34, 0x56]
    repeat: eos
