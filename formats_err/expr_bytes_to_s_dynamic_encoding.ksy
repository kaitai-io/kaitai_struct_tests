# expr_bytes_to_s_dynamic_encoding.ksy: /instances/bad/value:
# 	error: to_s: argument #0: expected string literal, got IfExp(Compare(Name(identifier(alpha)),Eq,IntNum(0)),Str(SJIS),Str(UTF-16LE))
#
meta:
  id: expr_bytes_to_s_dynamic_encoding
seq:
  - id: buf
    size: 5
  - id: alpha
    type: u1
instances:
  bad:
    value: |
      buf.to_s(alpha == 0 ? "SJIS" : "UTF-16LE")
