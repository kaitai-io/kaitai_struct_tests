# switch_cases_malformed_quoting2.ksy: /seq/1/type/cases/Name(identifier(AHEM)):
# 	error: invalid ID: 'AHEM', expected /^[a-z][a-z0-9_]*$/
#
meta:
  id: switch_cases_malformed_quoting2
seq:
  - id: code
    type: strz
    encoding: ASCII
  - id: foo
    type:
      switch-on: code
      cases:
        "AHEM": really
types:
  really: {}
