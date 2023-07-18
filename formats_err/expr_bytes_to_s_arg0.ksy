# expr_bytes_to_s_arg0.ksy: /instances/bad/value:
# 	error: to_s: expected 1 argument, got 0
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
