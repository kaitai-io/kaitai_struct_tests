# params_call_too_many_top_import.ksy: /seq/0/type:
# 	error: parameter count mismatch: 2 declared, but 3 used
#
meta:
  id: params_call_too_many_top_import
  imports:
    - params_def_top_imported
seq:
  - id: buf
    type: params_def_top_imported(2 + 3, true, "woohoo")
