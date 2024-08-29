# expr_broken_valid_eq_short.ksy: /seq/0/valid:
# 	error: parsing expression '1 *' failed on "*" at position 1:3, expected end-of-input
#
meta:
  id: expr_broken_valid_eq_short
seq:
  - id: foo
    type: u1
    valid: '1 *'
