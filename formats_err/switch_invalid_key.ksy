# switch_invalid_key.ksy: /seq/0/type/bar:
# 	error: unknown key found, expected: cases, switch-on
#
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
