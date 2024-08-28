# switch_on_malformed.ksy: /seq/0/type/switch-on:
# 	error: parsing expression '42/' failed on "/" at position 1:3, expected end-of-input
#
meta:
  id: switch_on_malformed
seq:
  - id: foo
    type:
      switch-on: 42/
      cases:
        42: really
types:
  really: {}
