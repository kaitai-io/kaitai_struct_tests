# expr_broken_kind_unbalanced.ksy: /seq/0/size:
# 	error: parsing expression '(1 + 5' failed on "(1 + 5" at position 1:1, expected (kw | comparison)
#
meta:
  id: expr_broken_kind_unbalanced
seq:
  - id: magic
    size: '(1 + 5'
