# switch_cases_invalid.ksy: /seq/0/type/cases:
# 	error: expected map, got foo (class java.lang.String)
#
meta:
  id: switch_cases_invalid
seq:
  - id: foo
    type:
      switch-on: 42
      cases: foo
types:
  really: {}
