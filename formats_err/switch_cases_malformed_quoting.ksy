# switch_cases_malformed_quoting.ksy: /seq/1/type/cases/^AHEM:
# 	error: parsing expression '^AHEM' failed on "^AHEM" at position 1:1, expected (kw | comparison)
#
meta:
  id: switch_cases_malformed_quoting
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
