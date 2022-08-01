# expr_sizeof_value_dynamic1.ksy: /instances/self_sizeof/value:
# 	error: unable to derive sizeof for type `expr_sizeof_value_dynamic1`: dynamic sized type
#
meta:
  id: expr_sizeof_value_dynamic1
  endian: le
seq:
  - id: len_body
    type: u1
  - id: body
    size: len_body
instances:
  self_sizeof:
    value: _sizeof
