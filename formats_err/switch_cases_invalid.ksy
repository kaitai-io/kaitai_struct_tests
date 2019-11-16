# /seq/0/cases: expected map, got foo (class java.lang.String)
meta:
  id: switch_cases_invalid
seq:
  - id: foo
    type:
      switch-on: 42
      cases: foo
types:
  really: {}
