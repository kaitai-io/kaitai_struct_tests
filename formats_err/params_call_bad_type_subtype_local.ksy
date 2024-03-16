# params_call_bad_type_subtype_local.ksy: /seq/0/type:
# 	error: can't pass argument #1 of type CalcFloatType into parameter `has_trailer` of type CalcBooleanType
#
meta:
  id: params_call_bad_type_subtype_local
seq:
  - id: buf
    type: my_str(5, 3.14159)
types:
  my_str:
    params:
      - id: len
        type: u4
      - id: has_trailer
        type: bool
    seq:
      - id: body
        type: str
        size: len
        encoding: UTF-8
      - id: trailer
        type: u1
        if: has_trailer
