# expr_sizeof_value_dynamic3.ksy: /instances/body_sizeof/value:
# 	error: unable to derive sizeof of `Name(identifier(body))`: dynamic sized type
#
meta:
  id: expr_sizeof_value_dynamic3
  endian: le
seq:
  - id: num_body
    type: u2
  - id: body
    type: u1
    repeat: expr
    repeat-expr: num_body
instances:
  body_sizeof:
    value: body._sizeof
