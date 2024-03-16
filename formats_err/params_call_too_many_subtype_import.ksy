# params_call_too_many_subtype_import.ksy: /seq/0/type:
# 	error: parameter count mismatch: 2 declared, but 3 used
#
meta:
  id: params_call_too_many_subtype_import
  imports:
    - params_def_subtype_imported
seq:
  - id: buf
    type: params_def_subtype_imported::my_str(2 + 3, true, "woohoo")
