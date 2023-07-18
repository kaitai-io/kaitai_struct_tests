# expr_bytes_to_s_arg2.ksy: /instances/bad/value:
# 	error: to_s: expected 1 argument, got 2
#
meta:
  id: expr_bytes_to_s_arg2
seq:
  - id: buf
    size: 5
instances:
  bad:
    value: |
      buf.to_s("foo", "bar")
