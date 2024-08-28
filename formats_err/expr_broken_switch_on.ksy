# expr_broken_switch_on.ksy: /seq/0/type/switch-on:
# 	error: parsing expression '42/' failed on "/" at position 1:3, expected end-of-input
#
meta:
  id: expr_broken_switch_on
seq:
  - id: foo
    type:
      switch-on: 42/
      cases:
        42: really
types:
  really: {}
