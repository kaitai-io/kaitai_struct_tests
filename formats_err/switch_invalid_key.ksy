# /seq/0/bar: unknown key found, expected: cases, switch-on
meta:
  id: switch_invalid_key
seq:
  - id: foo
    type:
      switch-on: 42
      cases:
        42: really
      bar: baz
types:
  really: {}
