# expr_broken_switch_cases.ksy: /seq/1/type/cases/^AHEM:
# 	error: parsing expression '^AHEM' failed on "^AHEM" at position 1:1, expected (kw | comparison)
#
meta:
  id: expr_broken_switch_cases
seq:
  - id: code
    type: strz
    encoding: ASCII
  - id: foo
    type:
      switch-on: code
      cases:
        "^AHEM": really
types:
  really: {}
