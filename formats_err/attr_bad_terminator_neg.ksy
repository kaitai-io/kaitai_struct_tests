# attr_bad_terminator_neg.ksy: /seq/0/terminator:
# 	error: expected an integer from 0 to 255, got -1
#
meta:
  id: attr_bad_terminator_neg
seq:
  - id: foo
    terminator: -1
