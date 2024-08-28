# expr_broken_kind_wrong_and.ksy: /instances/both/value:
# 	error: parsing expression 'foo == 1 && bar == 2' failed on "&& bar == " at position 1:10, expected end-of-input, did you mean 'and'?
#
meta:
  id: expr_broken_kind_wrong_and
seq:
  - id: foo
    type: u1
  - id: bar
    type: u1
instances:
  both:
    value: foo == 1 && bar == 2
