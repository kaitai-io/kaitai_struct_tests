# /seq/1/cases/^AHEM: parsing expression '^AHEM' failed on 1:1, expected "not" ~ not_test | comparison
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
