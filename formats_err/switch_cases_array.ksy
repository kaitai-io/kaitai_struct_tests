# /seq/0/cases: expected string, got [1, 2, 3] (class java.util.ArrayList)
meta:
  id: switch_on_array
seq:
  - id: foo
    type:
      switch-on: 42
      cases:
        [1, 2, 3]: really
types:
  really: {}
