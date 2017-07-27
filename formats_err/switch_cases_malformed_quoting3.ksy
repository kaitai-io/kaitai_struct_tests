# (main): /seq/1/type/cases/Name(identifier(ahem)): unable to access 'ahem' in switch_cases_malformed_quoting3 context
meta:
  id: switch_cases_malformed_quoting3
seq:
  - id: code
    type: strz
    encoding: ASCII
  - id: foo
    type:
      switch-on: code
      cases:
        "ahem": really
types:
  really: {}
