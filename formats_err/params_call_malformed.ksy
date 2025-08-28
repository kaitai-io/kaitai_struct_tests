# params_call_malformed.ksy: /seq/0/type:
# 	error: parsing expression '2 + 3, ' failed on 1:6, expected CharsWhile(Set( , n)) | "\\\n" | End
#
meta:
  id: params_call_malformed
seq:
  - id: buf
    type: my_str(2 + 3, )
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
