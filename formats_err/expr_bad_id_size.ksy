# expr_bad_id_size.ksy: /seq/0/size:
# 	error: invalid ID: 'BAD', expected /^[a-z][a-z0-9_]*$/
#
meta:
  id: expr_bad_id_size
seq:
  - id: foo
    size: BAD
