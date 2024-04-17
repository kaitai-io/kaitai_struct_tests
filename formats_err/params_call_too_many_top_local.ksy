# params_call_too_many_top_local.ksy: /seq/0/type:
# 	error: parameter count mismatch: 2 declared, but 3 used
#
meta:
  id: params_call_too_many_top_local
params:
  - id: len
    type: u4
  - id: has_trailer
    type: bool
seq:
  - id: buf
    type: params_call_too_many_top_local(2 + 3, true, "woohoo")
    if: false
