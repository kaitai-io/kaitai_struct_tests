# params_call_bad_type_top_local.ksy: /seq/0/type:
# 	error: can't pass argument #1 of type CalcFloatType into parameter `has_trailer` of type CalcBooleanType
#
meta:
  id: params_call_bad_type_top_local
params:
  - id: len
    type: u4
  - id: has_trailer
    type: bool
seq:
  - id: buf
    type: params_call_bad_type_top_local(5, 3.14159)
    if: false
