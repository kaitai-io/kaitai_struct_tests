# switch_cases_array.ksy: /seq/0/type/cases:
# 	error: expected string, got [1, 2, 3] (class java.util.ArrayList)
#
meta:
  id: switch_cases_array
seq:
  - id: foo
    type:
      switch-on: 42
      cases:
        [1, 2, 3]: really
types:
  really: {}
