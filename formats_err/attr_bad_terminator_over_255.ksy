# attr_bad_terminator_over_255.ksy: /seq/0/terminator:
# 	error: expected an integer from 0 to 255, got 256
#
meta:
  id: attr_bad_terminator_over_255
seq:
  - id: foo
    terminator: 256
