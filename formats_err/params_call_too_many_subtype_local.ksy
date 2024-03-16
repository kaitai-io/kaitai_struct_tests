# params_call_too_many_subtype_local.ksy: /seq/0/type:
# 	error: parameter count mismatch: 2 declared, but 3 used
#
meta:
  id: params_call_too_many_subtype_local
seq:
  - id: buf
    type: my_str(2 + 3, true, "woohoo")
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
