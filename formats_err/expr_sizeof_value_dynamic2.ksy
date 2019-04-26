# expr_sizeof_value_dynamic2: /instances/body_sizeof/value: unable to derive sizeof of `Name(identifier(body))`: dynamic sized type
meta:
  id: expr_sizeof_value_dynamic2
  endian: le
seq:
  - id: len
    type: u1
  - id: body
    size: len
instances:
  body_sizeof:
    value: body._sizeof
