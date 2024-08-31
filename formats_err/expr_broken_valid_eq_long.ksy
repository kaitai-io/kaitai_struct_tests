# expr_broken_valid_eq_long.ksy: /seq/0/valid/eq:
# 	error: parsing expression '1 *' failed on "*" at position 1:3, expected end-of-input
#
meta:
  id: expr_broken_valid_eq_long
seq:
  - id: foo
    type: u1
    valid:
      eq: '1 *'
