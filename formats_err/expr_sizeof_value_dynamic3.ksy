# ?
meta:
  id: expr_sizeof_value_dynamic3
  endian: le
seq:
  - id: num
    type: u2
  - id: body
    type: u1
    repeat: expr
    repeat-expr: num
instances:
  body_sizeof:
    value: body._sizeof
