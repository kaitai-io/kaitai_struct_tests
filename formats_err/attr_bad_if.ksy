# attr_bad_if.ksy: /seq/0/if:
# 	error: invalid type: expected boolean, got Int1Type(true)
#
meta:
  id: attr_bad_if
seq:
  - id: foo
    type: u1
    if: 5
