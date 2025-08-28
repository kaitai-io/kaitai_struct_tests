# params_call_bad_type_subtype_import.ksy: /seq/0/type:
# 	error: can't pass argument #1 of type CalcFloatType into parameter `has_trailer` of type CalcBooleanType
#
meta:
  id: params_call_bad_type_subtype_import
  imports:
    - params_def_subtype_imported
seq:
  - id: buf
    type: params_def_subtype_imported::my_str(5, 3.14159)
