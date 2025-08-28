# expr_bytes_to_s_arg0.ksy: /instances/bad/value:
# 	error: wrong arguments to method call `to_s` on byte array: expected (string), got ()
#
meta:
  id: expr_bytes_to_s_arg0
seq:
  - id: buf
    size: 5
instances:
  bad:
    value: |
      buf.to_s()
