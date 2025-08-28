# params_call_bad_type_top_import.ksy: /seq/0/type:
# 	error: can't pass argument #1 of type CalcFloatType into parameter `has_trailer` of type CalcBooleanType
#
meta:
  id: params_call_bad_type_top_import
  imports:
    - params_def_top_imported
seq:
  - id: buf
    type: params_def_top_imported(5, 3.14159)
