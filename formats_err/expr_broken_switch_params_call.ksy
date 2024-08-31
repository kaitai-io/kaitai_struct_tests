# expr_broken_switch_params_call.ksy: /seq/0/type/cases/IntNum(42):
# 	error: parsing expression 'really(' failed on "(" at position 1:7, expected end-of-input
#
meta:
  id: expr_broken_switch_params_call
seq:
  - id: foo
    type:
      switch-on: 42
      cases:
        42: really(
types:
  really: {}
