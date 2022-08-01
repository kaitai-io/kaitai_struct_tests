# expr_sizeof_value_dynamic2.ksy: /instances/body_sizeof/value:
# 	error: unable to derive sizeof of `Name(identifier(body))`: dynamic sized type
#
meta:
  id: expr_sizeof_value_dynamic2
  endian: le
seq:
  - id: len_body
    type: u1
  - id: body
    size: len_body
instances:
  body_sizeof:
    value: body._sizeof
