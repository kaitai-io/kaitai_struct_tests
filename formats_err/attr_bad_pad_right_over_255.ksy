# attr_bad_pad_right_over_255.ksy: /seq/0/pad-right:
# 	error: expected an integer from 0 to 255, got 256
#
meta:
  id: attr_bad_pad_right_over_255
seq:
  - id: foo
    size: 4
    pad-right: 256
