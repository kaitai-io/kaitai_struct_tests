# attr_bad_pad_right_neg.ksy: /seq/0/pad-right:
# 	error: expected an integer from 0 to 255, got -1
#
meta:
  id: attr_bad_pad_right_neg
seq:
  - id: foo
    size: 4
    pad-right: -1
